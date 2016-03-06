extern crate llamapun;
extern crate libxml;
extern crate senna;

use std::env;
use std::io::Write;
use std::collections::HashMap;

use libxml::xpath::Context;
use libxml::tree::Node;

use senna::sennapath::SENNA_PATH;
use senna::senna::{Senna, SennaParseOptions};

use llamapun::data::Corpus;
use llamapun::dnm::{DNM, DNMParameters};
use llamapun::tokenizer::Tokenizer;

fn get_parent_chain(from: &Node, until: &Node) -> Vec<Node> {
    let mut chain : Vec<Node> = Vec::new();
    let mut it = from.clone();
    while it != *until {
        chain.push(it.clone());
        it = it.get_parent().expect("Expected parent");
    }
    chain.push(it.clone());
    return chain;
}

fn is_child_of(child: &Node, parent: &Node, root: &Node) -> bool {
    let mut it = child.clone();
    loop {
        if it == *parent {
            return true;
        } else if it == *root {
            return false;
        } else {
            it = it.get_parent().expect("Expected parent");
        }
    }
}

fn get_plaintext(node: &Node) -> (String, Vec<usize>, Vec<Node>) {
    let mut plaintext = String::new();
    let mut offsets : Vec<usize> = Vec::new();
    let mut nodes : Vec<Node> = Vec::new();
    if node.is_text_node() {
        let content = node.get_content();
        for i in 0..content.len() {
            offsets.push(i);
            nodes.push(node.clone());
        }
        plaintext.push_str(&content);
    } else {
        let name = node.get_name();
        let classvals = node.get_class_names();
        if name == "math" || classvals.contains("ltx_equation") || classvals.contains("ltx_equationgroup") {
            plaintext.push_str("MathFormula");
            for _ in 0..11 {
                offsets.push(0);
                nodes.push(node.clone());
            }
        } else if name == "cite" {
            plaintext.push_str("CitationElement");
            for _ in 0..15 {
                offsets.push(0);
                nodes.push(node.clone());
            }
        } else if name == "table" {
            /* skip */
        } else {
            // recurse into children
            let mut child_opt = node.get_first_child();
            loop {
                match child_opt {
                    Some(child) => {
                        let (p, o, n) = get_plaintext(&child);
                        plaintext.push_str(&p);
                        // offsets.push_all(&o);
                        // nodes.push_all(&n);
                        offsets.extend(o.into_iter());
                        nodes.extend(n.into_iter());
                        child_opt = child.get_next_sibling();
                    },
                    None => break
                }
            }
        }
    }
    if plaintext.len() != offsets.len() || offsets.len() != nodes.len() {
        panic!("Lenghts don't match!!");
    }
    return (plaintext, offsets, nodes);
}



pub fn main() {
    let args : Vec<_> = env::args().collect();
    let corpus_path : &str = if args.len() > 1 { &args[1] } else { "tests/resources/" };
    println!("Loading corpus from \"{}\"", corpus_path);

    let mut senna = Senna::new(SENNA_PATH.to_owned());
    let tokenizer = Tokenizer::default();

    let mut corpus = Corpus::new(corpus_path.to_owned());
    for mut document in corpus.iter() {
        println!("Processing \"{}\"", &document.path);
        let mut dom = document.dom;
        let xpath_context = Context::new(&dom).unwrap();
        let paras = match xpath_context.evaluate("//*[contains(@class,'ltx_para')]") {
            Ok(result) => result.get_nodes_as_vec(),
            Err(_) => {
                writeln!(std::io::stderr(), "Warning: No paragraphs found").unwrap();
                vec![]
            }
        };

        for para in paras {
            let (plaintext, offsets, nodes) = get_plaintext(&para);
            // Need to create DNM for sentence tokenizer
            let dnm = DNM {
                plaintext : plaintext,
                parameters : DNMParameters::default(),
                root_node : para.clone(),
                node_map : HashMap::new(),
            };
            let sentences = tokenizer.sentences(&dnm);
            for sentence in sentences {
                // find lowest parent of range
                let (plaintext, offsets, nodes) = get_plaintext(&para); // Need to recalculate it every round
                let start_node = &nodes[sentence.start];
                let start_parents : Vec<Node> = get_parent_chain(start_node, &para);
                let end_node = &nodes[sentence.end-1];
                let end_parents : Vec<Node> = get_parent_chain(end_node, &para);


                let mut si = start_parents.len() - 1;
                let mut ei = end_parents.len() - 1;
                while si > 0 && ei > 0 && start_parents[si-1] == end_parents[ei-1] {
                    si -= 1;
                    ei -= 1;
                }
                
                let common_parent = &start_parents[si];

                if common_parent.is_text_node() {
                    // TODO: Simply split it
                } else if common_parent == start_node && common_parent == end_node {
                    // TODO: Annotate it or print warning
                } else {
                    // make sure splitting is possible
                    let mut act_start = start_parents[si - 1].clone();
                    if !(sentence.start == 0 ||
                           act_start.is_text_node() ||
                            !is_child_of(&nodes[sentence.start-1], &act_start, &para)) {

                        // let act_end = &end_parents[ei - 1];
                        // writeln!(std::io::stderr(), "NAMES: \"{}\" > \"{}\" | \"{}\"", common_parent.get_name(), act_start.get_name(), act_end.get_name()).unwrap();
                        writeln!(std::io::stderr(), "Warning: Couldn't split sentence (at beginning): \"{}\"",
                                                    sentence.get_plaintext()).unwrap();
                    }
                    let mut act_end = end_parents[ei - 1].clone();
                    if !(sentence.end == dnm.plaintext.len()-1 ||
                           act_end.is_text_node() ||
                            !is_child_of(&nodes[sentence.end+1], &act_end, &para)) {
                        writeln!(std::io::stderr(), "Warning: Couldn't split sentence (at end): \"{}\"",
                                                    sentence.get_plaintext()).unwrap();
                    }
                    
                    // split text nodes
                    if act_start.is_text_node() && offsets[sentence.start] != 0 {  // have to split act_start
                        let before = Node::new_text_node(&dom, &dnm.plaintext[sentence.start-offsets[sentence.start]..sentence.start]).unwrap();
                        let mut textend = sentence.start+1;
                        while offsets[textend] > 0 {  // can't run to end of array, because in that case we'd have a text node as common parent (checked for before)
                            textend += 1;
                        }
                        let after = Node::new_text_node(&dom, &dnm.plaintext[sentence.start..textend]).unwrap();
                        let break_ = Node::new("BREAK", None, &dom).unwrap();  // make sure text nodes don't get merged into act_start
                        act_start.add_prev_sibling(break_.clone()).unwrap();
                        break_.add_prev_sibling(before).unwrap();
                        let tmp = break_.add_prev_sibling(after).unwrap();
                        act_start.unlink();
                        act_start.free();
                        break_.unlink();
                        break_.free();
                        act_start = tmp;
                    }
                    if act_end.is_text_node() && sentence.end < dnm.plaintext.len() - 1 &&
                                         offsets[sentence.end+1] != 0 {
                        let before = Node::new_text_node(&dom, &dnm.plaintext[sentence.end - offsets[sentence.end]..sentence.end]).unwrap();
                        let mut textend = sentence.end + 1;
                        while textend < offsets.len() && offsets[textend] > 0 {
                            textend += 1;
                        }
                        let after = Node::new_text_node(&dom, &dnm.plaintext[sentence.end..textend]).unwrap();
                        let stop = Node::new("STOP", None, &dom).unwrap();
                        act_end.add_prev_sibling(stop.clone()).unwrap();
                        let tmp = stop.add_prev_sibling(before).unwrap();

                        stop.add_prev_sibling(after).unwrap();
                        act_end.unlink();
                        act_end.free();
                        stop.unlink();
                        stop.free();
                        act_end = tmp;
                    }

                    let snode = Node::new("span", None, &dom).unwrap();
                    snode.add_property("class", "sentence");
                    act_start.add_prev_sibling(snode.clone());
                    writeln!(std::io::stderr(), "END ({})", dom.node_to_string(&act_end)).unwrap();
                    while act_start != act_end {    // iterate with act_start to act_end and move everything inside snode
                        writeln!(std::io::stderr(), "OK ({})", &dnm.plaintext[sentence.start..sentence.end]).unwrap();
                        writeln!(std::io::stderr(), "OK ({})", dom.node_to_string(&act_start)).unwrap();
                        let tmp = act_start.get_next_sibling().unwrap();
                        act_start.unlink();
                        snode.add_child(&act_start);
                        act_start = tmp;
                    }
                    act_end.unlink();
                    snode.add_child(&act_end);
                    writeln!(std::io::stderr(), "DONE ({})", dom.node_to_string(&act_end)).unwrap();
                }
            }
        }
        dom.save_file("/tmp/test.html");
    }
}

