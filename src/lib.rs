use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Word {
    id: usize,
    form: String,
    lemma: String,
    upos: String,
    xpos: String,
    feats: HashMap<String, String>,
    head: usize,
    deprel: String,
    deps: HashMap<usize, String>,
    misc: HashMap<String, String>
}

pub fn parse_conllu(lines: impl Iterator<Item=&'static str>) -> Vec<Vec<Word>> {
    let mut ans = Vec::new();
    for line in lines {
        if line.starts_with("#") { continue; }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 10 { continue; }
        let id = fields[0].parse().unwrap();
        if id == 1 {
            ans.push(Vec::new());
        }
        ans.last_mut().unwrap().push(Word {
            id: id,
            form: fields[1].to_string(),
            lemma: fields[2].to_string(),
            upos: fields[3].to_string(),
            xpos: fields[4].to_string(),
            feats: parse_attrs(fields[5], '='),
            head: fields[6].parse().unwrap_or(0),
            deprel: fields[7].to_string(),
            deps: parse_attrs(fields[8], ':'),
            misc: parse_attrs(fields[9], '=')
        });
    }
    ans
}

fn parse_attrs<T>(text: &str, sep: char) -> HashMap<T, String>
where
    T: Eq,
    T: Hash,
    T: FromStr,
    <T as std::str::FromStr>::Err: Debug
{
    let mut ans = HashMap::new();
    if text == "_" { return ans; }
    for item in text.split('|') {
        let pair: Vec<&str> = item.split(sep).collect();
        if pair.len() != 2 { continue; }
        ans.insert(pair[0].parse().unwrap(), pair[1].to_string());
    }
    ans
}

#[cfg(test)]
mod tests {
    const TEST_DATA: &str = "# sent_id = 1
# text = They buy and sell books.
1	They	they	PRON	PRP	Case=Nom|Number=Plur	2	nsubj	2:nsubj|4:nsubj	_
2	buy	buy	VERB	VBP	Number=Plur|Person=3|Tense=Pres	0	root	0:root	_
3	and	and	CONJ	CC	_	4	cc	4:cc	_
4	sell	sell	VERB	VBP	Number=Plur|Person=3|Tense=Pres	2	conj	0:root|2:conj	_
5	books	book	NOUN	NNS	Number=Plur	2	obj	2:obj|4:obj	SpaceAfter=No
6	.	.	PUNCT	.	_	2	punct	2:punct	_

# sent_id = 2
# text = I have no clue.
1	I	I	PRON	PRP	Case=Nom|Number=Sing|Person=1	2	nsubj	_	_
2	have	have	VERB	VBP	Number=Sing|Person=1|Tense=Pres	0	root	_	_
3	no	no	DET	DT	PronType=Neg	4	det	_	_
4	clue	clue	NOUN	NN	Number=Sing	2	obj	_	SpaceAfter=No
5	.	.	PUNCT	.	_	2	punct	_	_";

    #[test]
    fn it_works() {
        let parsed = crate::parse_conllu(TEST_DATA.split('\n'));
        //println!("{:?}", parsed);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].len(), 6);
        assert_eq!(parsed[1].len(), 5);
    }
}
