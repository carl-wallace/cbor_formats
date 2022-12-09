use syn::Path;

pub(crate) fn extract_type(ty: &syn::Type) -> Option<String> {
    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    fn extract_last_segment(path: &Path) -> Option<String> {
        let mut idents_of_path =
            path.segments
                .iter()
                .into_iter()
                .fold(String::new(), |mut acc, v| {
                    acc.push_str(&v.ident.to_string());
                    acc.push('|');
                    acc
                });
        idents_of_path.pop();
        Some(idents_of_path)
    }

    match extract_type_from_option(ty) {
        Some(ty_from_opt) => match extract_type_from_vec(ty_from_opt) {
            Some(ty_from_opt_vec) => {
                extract_type_path(ty_from_opt_vec).and_then(extract_last_segment)
            }
            None => extract_type_path(ty_from_opt).and_then(extract_last_segment),
        },
        None => match extract_type_from_vec(ty) {
            Some(ty_from_vec) => extract_type_path(ty_from_vec).and_then(extract_last_segment),
            None => extract_type_path(ty).and_then(extract_last_segment),
        },
    }
}
//adapted this from https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
pub(crate) fn extract_type_from_vec(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::{GenericArgument, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    fn extract_vec_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        vec!["Vec|", "std|vec|Vec|", "core|vec|Vec|"]
            .into_iter()
            .find(|s| idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(extract_vec_segment)
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

pub(crate) fn is_option_vec(ty: &syn::Type) -> bool {
    match extract_type_from_option(ty) {
        Some(ty_from_opt) => extract_type_from_vec(ty_from_opt).is_some(),
        None => false,
    }
}

//adapted this from https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
pub(crate) fn is_vec(ty: &syn::Type) -> bool {
    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    fn extract_option_segment(path: &Path) -> Option<bool> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        let b = vec!["Vec|", "std|vec|Vec|", "core|vec|Vec|"]
            .into_iter()
            .any(|s| idents_of_path == s);
        if b {
            Some(b)
        } else {
            None
        }
    }

    extract_type_path(ty)
        .and_then(extract_option_segment)
        .is_some()
}

//adapted this from https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
pub(crate) fn is_option(ty: &syn::Type) -> bool {
    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    fn extract_option_segment(path: &Path) -> Option<bool> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        let b = vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .into_iter()
            .any(|s| idents_of_path == s);
        if b {
            Some(b)
        } else {
            None
        }
    }

    extract_type_path(ty)
        .and_then(extract_option_segment)
        .is_some()
}

// poached this from https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
pub(crate) fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::{GenericArgument, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .into_iter()
            .find(|s| idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(extract_option_segment)
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
