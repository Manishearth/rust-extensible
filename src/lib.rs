#![feature(plugin_registrar, box_syntax)]

#![feature(rustc_private)]

#[macro_use]
extern crate syntax;
#[macro_use]
extern crate rustc;

use rustc::plugin::Registry;
use rustc::lint::LintPassObject;

use syntax::ast;
use rustc::lint::{Context, LintPass, LintArray};
use rustc::middle::ty;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box ExtensiblePass as LintPassObject);
}

#[allow(missing_copy_implementations)]
/// A lint pass which catches uses of extensible enums in matches lacking wildcards
pub struct ExtensiblePass;


declare_lint!(pub EXTENSIBLE_ENUM, Forbid,
              "Warn on matches on extensible enums without wildcards");

impl LintPass for ExtensiblePass {
    fn get_lints(&self) -> LintArray {
        lint_array!(EXTENSIBLE_ENUM)
    }

    fn check_expr(&mut self, cx: &Context, expr: &ast::Expr) {
        // If it's a normal match ...
        if let ast::ExprMatch(ref ex, ref arms, ast::MatchSource::Normal) = expr.node {
            let e_ty = cx.tcx.expr_ty(&*ex);
            // ... on an enum type with #[extensible] ...
            match  e_ty.sty {
                ty::TyEnum(did, _) if cx.tcx.has_attr(did, "extensible") => (),
                _ => return
            }
            // ... which has at least one arm ..
            for ref arm in arms {
                if arm.guard.is_some() {
                    // ... which has no guards ..
                    continue;
                }
                for pat in arm.pats.iter() {
                    match pat.node {
                        // ... and is a form of wildcard ...
                        // ... then it's okay
                        ast::PatWild(_) | ast::PatLit(_) => return,
                        _ => ()
                    }
                }
            }
            // else, error
            cx.span_lint(EXTENSIBLE_ENUM, expr.span, &format!("The enum {} is marked as extensible, \
                                                              and may increase in size in the future. \
                                                              Please add a wildcard arm to \
                                                              this match statement",
                                                              e_ty)[..])
        }
    }
}