use syn::punctuated::Punctuated;
use syn::*;

pub trait IExprCallFactory {
    fn create(&self, func: Expr, arg: Expr) -> Expr;

    fn create_with_args(&self, func: Expr, args: Vec<Expr>) -> Expr;

    fn create_without_args(&self, func: Expr) -> Expr;
}

pub(crate) struct ExprCallFactory;

impl IExprCallFactory for ExprCallFactory {
    fn create(&self, func: Expr, arg: Expr) -> Expr {
        let result = Expr::Call(ExprCall {
            attrs: Vec::new(),
            func: Box::new(func),
            paren_token: Default::default(),
            args: [arg].into_iter().collect(),
        });
        return result;
    }

    fn create_with_args(&self, func: Expr, args: Vec<Expr>) -> Expr {
        let result = Expr::Call(ExprCall {
            attrs: Vec::new(),
            func: Box::new(func),
            paren_token: Default::default(),
            args: args.into_iter().collect(),
        });
        return result;
    }

    fn create_without_args(&self, func: Expr) -> Expr {
        let result = Expr::Call(ExprCall {
            attrs: Vec::new(),
            func: Box::new(func),
            paren_token: Default::default(),
            args: Punctuated::new(),
        });
        return result;
    }
}
