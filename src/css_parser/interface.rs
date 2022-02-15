#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct Selector {
    /// Can be '*' for universal
    pub tag_name: Option<String>,
    /// #...
    pub identifier: Option<String>,
    /// .x.y.z
    pub class_names: Option<Vec<String>>,
    /// div h1
    pub descendant: Option<Box<Selector>>,
    /// div > h1
    pub child: Option<Box<Selector>>,
}

impl Selector {
    pub fn new() -> Selector {
        return Selector {
            tag_name: None,
            identifier: None,
            class_names: None,
            descendant: None,
            child: None,
        };
    }
}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub value: CSSValue,
}

impl Declaration {
    pub fn new() -> Declaration {
        return Declaration {
            name: String::new(),
            value: CSSValue::Keyword(String::new()),
        };
    }
}

#[derive(Debug)]
pub enum CSSValue {
    Keyword(String),
    Function(String, Vec<CSSValue>),
    StringLiteral(String),
    Number(Number),
    NumberWithUnit(Number, String),
    Percentage(Number),
    Color(String),
    List(Vec<CSSValue>),
    CommaSeparatedList(Vec<CSSValue>),
}

#[derive(Debug)]
pub struct Number(pub String);
