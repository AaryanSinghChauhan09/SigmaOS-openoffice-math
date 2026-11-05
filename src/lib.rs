#[derive(Debug, Clone, PartialEq)]
pub enum MathNode {
    Number(f64),
    Symbol(String),
    Fraction(Box<MathNode>, Box<MathNode>),
    Exponent(Box<MathNode>, Box<MathNode>),
}

pub struct MathFormula {
    pub root: MathNode,
}

impl MathFormula {
    pub fn fraction(num: MathNode, den: MathNode) -> Self {
        Self { root: MathNode::Fraction(Box::new(num), Box::new(den)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_formula() {
        // Arrange & Act
        let f = MathFormula::fraction(MathNode::Number(1.0), MathNode::Symbol("x".into()));
        // Assert
        match f.root {
            MathNode::Fraction(..) => assert!(true),
            _ => panic!("Expected fraction"),
        }
    }
}
