use std::str::FromStr;
use std::fmt::Debug;
use tendril::Tendril;
use tendril::fmt::UTF8;
use tendril::SliceExt;

#[derive(Debug, Clone)]
pub struct Word {
    pub id: usize,
    pub form: Tendril<UTF8>,
    pub lemma: Tendril<UTF8>,
    pub upos: Tendril<UTF8>,
    pub xpos: Tendril<UTF8>,
    pub feats: Vec<(Tendril<UTF8>, Tendril<UTF8>)>,
    pub head: usize,
    pub deprel: Tendril<UTF8>,
    pub deps: Vec<(usize, Tendril<UTF8>)>,
    pub misc: Vec<(Tendril<UTF8>, Tendril<UTF8>)>
}

pub fn parse_conllu(lines: impl Iterator<Item=String>) -> Vec<Vec<Word>> {
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
            form: fields[1].to_tendril(),
            lemma: fields[2].to_tendril(),
            upos: fields[3].to_tendril(),
            xpos: fields[4].to_tendril(),
            feats: parse_attrs(fields[5], '='),
            head: fields[6].parse().unwrap_or(0),
            deprel: fields[7].to_tendril(),
            deps: parse_attrs(fields[8], ':'),
            misc: parse_attrs(fields[9], '=')
        });
    }
    ans
}

fn parse_attrs<T>(text: &str, sep: char) -> Vec<(T, Tendril<UTF8>)>
where
    T: Eq,
    T: FromStr,
    <T as std::str::FromStr>::Err: Debug
{
    let mut ans = Vec::new();
    if text == "_" { return ans; }
    for item in text.split('|') {
        let pair: Vec<&str> = item.split(sep).collect();
        if pair.len() != 2 { continue; }
        ans.push((pair[0].parse().unwrap(), pair[1].to_tendril()));
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
        let parsed = crate::parse_conllu(TEST_DATA.split('\n').map(String::from));
        //println!("{:?}", parsed);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].len(), 6);
        assert_eq!(parsed[1].len(), 5);
    }
}
