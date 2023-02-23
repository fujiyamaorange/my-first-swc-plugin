use convert_case::{Case, Casing};
use swc_core::ecma::{
    ast::{Pat, Program, VarDecl},
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

/**
 * Convert to snake-case
 */
fn convert_to_snake_case(s: String) -> String {
    return s.to_case(Case::Snake).to_lowercase();
}

impl VisitMut for TransformVisitor {
    fn visit_mut_var_decl(&mut self, n: &mut VarDecl) {
        let decls = &mut n.decls;

        for decl in decls.iter_mut() {
            let name = &mut decl.name;

            if let Pat::Ident(binding_ident) = name {
                let id = &mut binding_ident.id;
                id.sym = convert_to_snake_case(id.sym.to_string()).into();
            }
        }
    }
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    test1,
    // Input codes
    r#"
    const yourName = "KIMINONAWA";
    "#,
    // Output codes after transformed with plugin
    r#"
    const your_name = "KIMINONAWA";
    "#
);
