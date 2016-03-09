use std::collections::HashSet;

// Annoyingly, HashSets are not allowed as static variables in Rust (yet?)
pub fn load<'a>() -> HashSet<&'a str> {
  // math_basic_stopwords = ["'t", "doesn", "wasn", "isn", "wouldn", "a","able","about","above","abroad","according","accordingly","across","actually","adj","after","afterwards","again","against","ago","ahead","ain't","al","all","almost","alone","along","alongside","already","also","although","always","am","amid","amidst","among","amongst","an","and","another","any","anybody","anyhow","anyone","anything","anyway","anyways","anywhere","apart","appropriate","are","aren't","around","a's","as","aside","at","available","away","awfully","backward","backwards","be","because","been","before","beforehand","behind","being","believe","below","beside","besides","best","better","between","beyond","both","brief","but","by","come","came","can","cannot","cant","caption","certain","certainly","clearly","c'mon","co.","co","com","come","comes","corresponding","could","couldn't","course","c's","currently","dare","daren't","definitely","despite","did","didn't","different","directly","do","does","doesn't","doing","done","don't","down","downwards","during","each","easy","edu","eg","eight","eighty","either","else","elsewhere","ending","enough","entirely","entry","especially","et","etc","ever","evermore","every","everybody","everyone","everything","everywhere","ex","exactly","except","fairly","far","farther","few","fewer","for","forever","former","formerly","forth","forward","from","further","furthermore","get","gets","getting","give","given","gives","goes","going","gone","got","gotten","greetings","had","hadn't","happens","hardly","has","hasn't","have","haven't","having","he","he'd","he'll","hello","help","her","here","here's","hers","herself","he's","hi","him","himself","his","hither","hopefully","how","howbeit","however","i","i'd","ie","if","i'll","i'm","immediate","in","inasmuch","inc.","inc","indeed","inside","insofar","instead","into","inward","is","isn't","it","it'd","it'll","it's","its","itself","i've","just","keep","keeps","kept","know","known","knows","last","lately","later","latter","latterly","least","less","lest","let","let's","like","liked","likely","likewise","look","looking","looks","ltd","made","mainly","make","makes","many","may","maybe","mayn't","me","meantime","meanwhile","merely","might","mightn't","mine","miss","more","moreover","most","mostly","mr","mrs","much","must","mustn't","my","myself","name","namely","nd","near","nearly","necessary","need","needn't","needs","neither","never","neverf","neverless","nevertheless","new","next","nine","ninety","no","nobody","non","none","nonetheless","no-one","noone","nor","normally","not","notwithstanding","novel","now","nowhere","obviously","of","off","often","oh","ok","okay","old","on","once","one","one's","ones","only","onto","opposite","or","originally","other","others","otherwise","ought","oughtn't","our","ours","ourselves","out","outside","over","overall","own","particular","particularly","past","per","perhaps","placed","please","possible","presumably","probably","que","quite","qv","rather","rd","re","really","reasonably","recent","recently","reference","regarding","regardless","regards","relatively","required","respective","respectively","said","same","saw","say","saying","says","secondly","see","seeing","seem","seemed","seeming","seems","seen","self","selves","sensible","sent","serious","seriously","seven","several","shall","shan't","she","she'd","she'll","she's","show","shows","showed","should","shouldn't","side","similarly","since","six","so","some","somebody","someday","somehow","someone","something","sometime","sometimes","somewhat","somewhere","soon","sorry","specified","still","sub","such","sup","sure","tell","th","than","thank","thanks","thanx","that","that'll","that's","thats","that've","the","their","theirs","them","themselves","then","thence","there","thereafter","thereby","there'd","therefore","therein","there'll","there're","there's","theres","thereupon","there've","these","they","they'd","they'll","they're","they've","thing","things","think","thirty","this","thorough","thoroughly","those","though","three","through","throughout","thru","thus","till","to","together","too","took","toward","towards","tried","tries","truly","try","trying","t's","twice","two","un","under","underneath","undoing","unfortunately","unless","unlike","unlikely","until","unto","up","upon","upwards","us","use","used","useful","uses","using","usually","various","versus","very","via","viz","vs","want","wants","was","wasn't","way","we","we'd","welcome","we'll","well","went","we're","were","weren't","we've","what","whatever","what'll","what's","what've","when","whence","whenever","where","whereafter","whereas","whereby","wherein","where's","whereupon","wherever","whether","which","whichever","while","whilst","whither","who","who'd","whoever","whole","who'll","whom","whomever","who's","whose","why","will","willing","wish","with","within","without","wonder","won't","work","would","wouldn't","write","written","yes","yet","you","you'd","you'll","your","you're","yours","yourself","yourselves","you've"].iter().cloned().collect();
  // math_stopwords = ["'t", "doesn", "wasn", "isn", "wouldn", "a","able","about","above","abroad","according","accordingly","across","actually","adj","after","afterwards","again","against","ago","ahead","ain't","al","all","allow","allows","almost","alone","along","alongside","already","also","although","always","am","amid","amidst","among","amongst","an","and","another","any","anybody","anyhow","anyone","anything","anyway","anyways","anywhere","apart","appear","appreciate","appropriate","arbitrary","are","aren't","around","a's","as","aside","ask","asking","associated","at","available","away","awfully","backward","backwards","be","became","because","become","becomes","becoming","been","before","beforehand","begin","behind","being","believe","below","beside","besides","best","better","between","beyond","both","brief","but","by","call","called","came","can","cannot","can't","cant","caption","cause","causes","certain","certainly","case","changes","clearly","c'mon","co.","co","com","come","comes","concerning","consequently","consider","considering","consist","consisting","contain","containing","contains","corresponding","could","couldn't","course","c's","currently","dare","daren't","definitely","defined","denote","denoted","described","despite","did","didn't","different","directly","do","does","doesn't","doing","done","don't","down","downwards","during","each","easy","edu","eg","eight","eighty","either","else","elsewhere","ending","enough","entirely","entry","especially","et","etc","ever","evermore","every","everybody","everyone","everything","everywhere","ex","exactly","example","except","expressed","express","fairly","far","farther","few","fewer","followed","following","follows","for","forever","former","formerly","forth","forward","found","from","further","furthermore","get","gets","getting","give","given","gives","goes","going","gone","got","gotten","greetings","had","hadn't","happens","hardly","has","hasn't","have","haven't","having","he","he'd","he'll","hello","help","hence","her","here","hereafter","hereby","herein","here's","hereupon","hers","herself","he's","hi","him","himself","his","hither","hopefully","how","howbeit","however","i","i'd","ie","if","ignored","i'll","i'm","immediate","in","inasmuch","inc.","inc","include","includes","indeed","indicate","indicated","indicates","inside","insofar","instead","into","inward","is","isn't","it","it'd","it'll","it's","its","itself","i've","just","keep","keeps","kept","know","known","knows","last","lately","later","latter","latterly","least","less","lest","let","let's","like","liked","likely","likewise","look","looking","looks","ltd","made","mainly","make","makes","many","may","maybe","mayn't","me","meantime","meanwhile","merely","might","mightn't","mine","miss","more","moreover","most","mostly","mr","mrs","much","must","mustn't","my","myself","name","namely","nd","near","nearly","necessary","need","needn't","needs","neither","never","neverf","neverless","nevertheless","new","next","nine","ninety","no","nobody","non","none","nonetheless","no-one","noone","nor","normally","not","note","notion","nothing","notwithstanding","novel","now","nowhere","obtain","obtained","obviously","of","off","often","oh","ok","okay","old","on","once","one","one's","ones","only","onto","opposite","or","originally","other","others","otherwise","ought","oughtn't","our","ours","ourselves","out","outside","over","overall","own","particular","particularly","past","per","perhaps","placed","please","possible","presumably","probably","prove","proves","proved","provided","provides","que","quite","qv","rather","rd","re","really","reasonably","recent","recently","reference","regarding","regardless","regards","relatively","required","respective","respectively","said","same","saw","say","saying","says","secondly","see","seeing","seem","seemed","seeming","seems","seen","self","selves","sensible","sent","serious","seriously","seven","several","shall","shan't","she","she'd","she'll","she's","show","shows","showed","should","shouldn't","side","similarly","since","six","so","solve","solving","solved","some","somebody","someday","somehow","someone","something","sometime","sometimes","somewhat","somewhere","soon","sorry","specified","specify","specifying","still","sub","such","sup","suppose","sure","take","taken","taking","tell","tends","th","than","thank","thanks","thanx","that","that'll","that's","thats","that've","the","their","theirs","them","themselves","then","thence","there","thereafter","thereby","there'd","therefore","therein","there'll","there're","there's","theres","thereupon","there've","these","they","they'd","they'll","they're","they've","thing","things","think","thirty","this","thorough","thoroughly","those","though","three","through","throughout","thru","thus","till","to","together","too","took","toward","towards","tried","tries","truly","try","trying","t's","twice","two","un","under","underneath","undoing","unfortunately","unless","unlike","unlikely","until","unto","up","upon","upwards","us","use","used","useful","uses","using","usually","various","versus","very","via","viz","vs","want","wants","was","wasn't","way","we","we'd","welcome","we'll","well","went","we're","were","weren't","we've","what","whatever","what'll","what's","what've","when","whence","whenever","where","whereafter","whereas","whereby","wherein","where's","whereupon","wherever","whether","which","whichever","while","whilst","whither","who","who'd","whoever","whole","who'll","whom","whomever","who's","whose","why","will","willing","wish","with","within","without","wonder","won't","work","would","wouldn't","write","written","yes","yet","you","you'd","you'll","your","you're","yours","yourself","yourselves","you've"].iter().cloned().collect();
  return ["'t", "doesn", "wasn", "isn", "wouldn", "a","able","about","above","abroad","according","accordingly","across","actually","adj","after","afterwards","again","against","ago","ahead","ain't","al","all","allow","allows","almost","alone","along","alongside","already","also","although","always","am","amid","amidst","among","amongst","an","and","another","any","anybody","anyhow","anyone","anything","anyway","anyways","anywhere","apart","appear","appreciate","appropriate","arbitrary","are","aren't","around","a's","as","aside","ask","asking","associated","at","available","away","awfully","backward","backwards","be","became","because","become","becomes","becoming","been","before","beforehand","begin","behind","being","believe","below","beside","besides","best","better","between","beyond","both","brief","but","by","call","called","came","can","cannot","can't","cant","caption","cause","causes","certain","certainly","case","changes","clearly","c'mon","co.","co","com","come","comes","concerning","consequently","consider","considering","consist","consisting","contain","containing","contains","corresponding","could","couldn't","course","c's","currently","dare","daren't","definitely","defined","denote","denoted","described","despite","did","didn't","different","directly","do","does","doesn't","doing","done","don't","down","downwards","during","each","easy","edu","eg","eight","eighty","either","else","elsewhere","ending","enough","entirely","entry","especially","et","etc","ever","evermore","every","everybody","everyone","everything","everywhere","ex","exactly","example","except","expressed","express","fairly","far","farther","few","fewer","followed","following","follows","for","forever","former","formerly","forth","forward","found","from","further","furthermore","get","gets","getting","give","given","gives","goes","going","gone","got","gotten","greetings","had","hadn't","happens","hardly","has","hasn't","have","haven't","having","he","he'd","he'll","hello","help","hence","her","here","hereafter","hereby","herein","here's","hereupon","hers","herself","he's","hi","him","himself","his","hither","hopefully","how","howbeit","however","i","i'd","ie","if","ignored","i'll","i'm","immediate","in","inasmuch","inc.","inc","include","includes","indeed","indicate","indicated","indicates","inside","insofar","instead","into","inward","is","isn't","it","it'd","it'll","it's","its","itself","i've","just","keep","keeps","kept","know","known","knows","last","lately","later","latter","latterly","least","less","lest","let","let's","like","liked","likely","likewise","look","looking","looks","ltd","made","mainly","make","makes","many","may","maybe","mayn't","me","meantime","meanwhile","merely","might","mightn't","mine","miss","more","moreover","most","mostly","mr","mrs","much","must","mustn't","my","myself","name","namely","nd","near","nearly","necessary","need","needn't","needs","neither","never","neverf","neverless","nevertheless","new","next","nine","ninety","no","nobody","non","none","nonetheless","no-one","noone","nor","normally","not","note","notion","nothing","notwithstanding","novel","now","nowhere","obtain","obtained","obviously","of","off","often","oh","ok","okay","old","on","once","one","one's","ones","only","onto","opposite","or","originally","other","others","otherwise","ought","oughtn't","our","ours","ourselves","out","outside","over","overall","own","particular","particularly","past","per","perhaps","placed","please","possible","presumably","probably","prove","proves","proved","provided","provides","que","quite","qv","rather","rd","re","really","reasonably","recent","recently","reference","regarding","regardless","regards","relatively","required","respective","respectively","said","same","saw","say","saying","says","secondly","see","seeing","seem","seemed","seeming","seems","seen","self","selves","sensible","sent","serious","seriously","seven","several","shall","shan't","she","she'd","she'll","she's","show","shows","showed","should","shouldn't","side","similarly","since","six","so","solve","solving","solved","some","somebody","someday","somehow","someone","something","sometime","sometimes","somewhat","somewhere","soon","sorry","specified","specify","specifying","still","sub","such","sup","suppose","sure","take","taken","taking","tell","tends","th","than","thank","thanks","thanx","that","that'll","that's","thats","that've","the","their","theirs","them","themselves","then","thence","there","thereafter","thereby","there'd","therefore","therein","there'll","there're","there's","theres","thereupon","there've","these","they","they'd","they'll","they're","they've","thing","things","think","thirty","this","thorough","thoroughly","those","though","three","through","throughout","thru","thus","till","to","together","too","took","toward","towards","tried","tries","truly","try","trying","t's","twice","two","un","under","underneath","undoing","unfortunately","unless","unlike","unlikely","until","unto","up","upon","upwards","us","use","used","useful","uses","using","usually","various","versus","very","via","viz","vs","want","wants","was","wasn't","way","we","we'd","welcome","we'll","well","went","we're","were","weren't","we've","what","whatever","what'll","what's","what've","when","whence","whenever","where","whereafter","whereas","whereby","wherein","where's","whereupon","wherever","whether","which","whichever","while","whilst","whither","who","who'd","whoever","whole","who'll","whom","whomever","who's","whose","why","will","willing","wish","with","within","without","wonder","won't","work","would","wouldn't","write","written","yes","yet","you","you'd","you'll","your","you're","yours","yourself","yourselves","you've","inspired"].iter().cloned().collect();
}