/// Utils for parsing Syn output.
/// TODO: Consolidate all the util functions here (maybe use some other proc_macros?)
// Extracts the type from an Option
// ref: https://stackoverflow.com/questions/63878327/how-can-a-procedural-macro-check-a-generic-type-for-optionoptiont-and-flatte
use syn::{GenericArgument, Path, PathArguments, PathSegment};

pub fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
    match *ty {
        syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
        _ => None,
    }
}

pub fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
    let idents_of_path = path.segments.iter().into_iter().fold(String::new(), |mut acc, v| {
        acc.push_str(&v.ident.to_string());
        acc.push('|');
        acc
    });
    vec!["Option|", "std|option|Option|", "core|option|Option|"]
        .into_iter()
        .find(|s| &idents_of_path == *s)
        .and_then(|_| path.segments.last())
}

pub(crate) fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    extract_type_path(ty)
        .and_then(|path| extract_option_segment(path))
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}
