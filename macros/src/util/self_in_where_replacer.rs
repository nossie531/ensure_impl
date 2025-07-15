//! Provider of [`SelfInWhereReplacer`].

use proc_macro2::Span;
use syn::visit_mut::{self, VisitMut};

/// `Self` keyword replacer in `where`.
pub struct SelfInWhereReplacer<'a> {
    /// `where` clause.
    wc: &'a syn::WhereClause,
    /// Alternate type for `Self`.
    alt: &'a syn::Type,
}

impl<'a> SelfInWhereReplacer<'a> {
    /// Creates a new value.
    pub fn new(wc: &'a syn::WhereClause, alt: &'a syn::Type) -> Self {
        Self { wc, alt }
    }

    /// Executes replace.
    pub fn exec(mut self) -> syn::WhereClause {
        let mut where_clause = self.wc.clone();
        self.visit_where_clause_mut(&mut where_clause);
        where_clause
    }
}

impl VisitMut for SelfInWhereReplacer<'_> {
    fn visit_type_mut(&mut self, i: &mut syn::Type) {
        let self_id = &syn::Ident::new("Self", Span::call_site());
        if let syn::Type::Path(type_path) = i {
            if type_path.path.get_ident() == Some(self_id) {
                *i = self.alt.clone();
            }
        }

        visit_mut::visit_type_mut(self, i);
    }
}
