use crate::MapT;
use crate::gon_grammar_trait::{Bool, GonGrammarTrait, ListSuffix, ObjSuffix, Pair, Value};
use crate::value::Value as GonValue;
#[allow(unused_imports)]
use parol_runtime::{Result, Token};
use std::fmt::{Debug, Display, Error, Formatter};

///
/// Data structure that implements the semantic actions for our gon grammar
/// !Change this type as needed!
///
#[derive(Debug, Default)]
pub struct GonGrammar<'t> {
    pub value: Option<Value<'t>>,
}

impl<'t> From<Value<'t>> for GonValue {
    fn from(value: Value<'t>) -> Self {
        match value {
            Value::None(_) => GonValue::None,
            Value::Str(s) => GonValue::Str(s.str.str.text()[1..s.str.str.text().len() - 1].into()),
            Value::Num(n) => GonValue::Num(String::from(n.num.num.text())),
            Value::Bool(b) => GonValue::Bool(matches![b.bool.as_ref(), Bool::True(_)]),
            Value::Obj(obj) => {
                let obj = obj.obj.obj_suffix;
                match obj.as_ref() {
                    ObjSuffix::PairObjListTrailingCommaRBrace(o) => {
                        let mut map = MapT::new();
                        insert_pair_into_map(*o.pair.clone(), &mut map);
                        for pair in &o.obj_list {
                            insert_pair_into_map(*pair.pair.clone(), &mut map);
                        }
                        GonValue::Obj(map)
                    }
                    ObjSuffix::RBrace(_) => GonValue::Obj(MapT::new()),
                }
            }
            Value::List(l) => {
                let l = l.list.list_suffix;
                match l.as_ref() {
                    ListSuffix::ValueListListTrailingCommaRBracket(l) => {
                        let mut list = Vec::new();
                        list.push(GonValue::from(*l.value.clone()));
                        for val in &l.list_list {
                            list.push(GonValue::from(*val.value.clone()));
                        }
                        GonValue::List(list)
                    }
                    ListSuffix::RBracket(_) => GonValue::List(Vec::new()),
                }
            }
        }
    }
}

fn insert_pair_into_map(pair: Pair, map: &mut MapT) {
    map.insert(
        pair.l_bracket_circumflex_colon_l_bracket_r_bracket_l_brace_r_brace_comma_s_r_bracket_plus
            .text()
            .to_owned(),
        GonValue::from(*pair.value),
    );
}

impl GonGrammar<'_> {
    pub fn new() -> Self {
        GonGrammar::default()
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for GonGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.value {
            Some(gon) => writeln!(f, "{}", gon),
            None => write!(f, "No parse result"),
        }
    }
}

impl<'t> GonGrammarTrait<'t> for GonGrammar<'t> {
    // !Adjust your implementation as needed!

    /// Semantic action for non-terminal 'gon'
    fn value(&mut self, arg: &Value<'t>) -> Result<()> {
        self.value = Some(arg.clone());
        Ok(())
    }
}
