#![feature(plugin_registrar, box_syntax)]

#![feature(core, rustc_private)]

#[macro_use]
extern crate syntax;
#[macro_use]
extern crate rustc;


use rustc::plugin::Registry;
use rustc::lint::LintPassObject;

use syntax::ast;
use rustc::lint::{Context, LintPass, LintArray};
use rustc::middle::ty;
use rustc::util::ppaux::Repr;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box ExtensiblePass as LintPassObject);
}

#[allow(missing_copy_implementations)]
pub struct ExtensiblePass;


declare_lint!(pub EXTENSIBLE_ENUM, Forbid,
              "Warn on matches on extensible enums without wildcards");

impl LintPass for ExtensiblePass {
    fn get_lints(&self) -> LintArray {
        lint_array!(EXTENSIBLE_ENUM)
    }

    fn check_expr(&mut self, cx: &Context, expr: &ast::Expr) {
        if let ast::ExprMatch(ref ex, ref arms, ast::MatchSource::Normal) = expr.node {
            let e_ty = ty::expr_ty(cx.tcx, &*ex);
            match  e_ty.sty {
                ty::ty_enum(did, _) if ty::has_attr(cx.tcx, did, "extensible") => (),
                _ => return
            }
            for ref arm in arms {
                if arm.guard.is_some() {
                    continue;
                }
                for pat in arm.pats.iter() {
                    match pat.node {
                        ast::PatWild(_) | ast::PatLit(_) => return,
                        _ => ()
                    }
                }
            }
            cx.span_lint(EXTENSIBLE_ENUM, expr.span, format!("The enum {} is marked as extensible, and may increase \
                                                              in size in the future. Please add a wildcard arm to \
                                                              this match statement", e_ty.repr(cx.tcx)).as_slice())
        }
    }
}