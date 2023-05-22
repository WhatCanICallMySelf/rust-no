use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Feil" => "Err",
        "Okei" => "Ok",
        "Streng" => "String",
        "Hashtabel" => "HashMap",
        "Standard" => "Default",
        "Feilfunktion" => "Error",
        "Kanske" => "Option",
        "Noe" => "Some",
        "Ingenting" => "None",
        "Resultat" => "Result",
        "Selv" => "Self",
        "skrivln" => "println",
        "bryt" => "break",
        "asynkron" => "async",
        "vent" => "await",
        "løkke" => "loop",
        "flytta" => "move",
        "lag" => "crate",
        "utilgjengelig_kode" => "unreachable_code",
        "som" => "as",
        "konstant" => "const",
        "trekk" => "trait",
        "usikker" => "unsafe",
        "av" => "in",
        "fra" => "from",
        "dynamisk" => "dyn",
        "pakk_ut" => "unwrap",
        "standard" => "default",
        "som_ref" => "as_ref",
        "io" => "io",
        "ekstern" => "extern",
        "falsk" => "false",
        "funksjon" => "fn",
        "over" => "super",
        "sett_inn" => "insert",
        "hent" => "get",
        "tillat" => "allow",
        "faen" | "helvete" | "dritt" => "panic",
        "module" => "mod",
        "foranderlig" => "mut",
        "ny" => "new",
        "hvor" => "where",
        "for" => "for",
        "ta_eller_sett_inn_med" => "get_or_insert_with",
        "hoved" => "main",
        "offentlig" => "pub",
        "hva" => None?,
        "returner" => "return",
        "implementere" => "impl",
        "referanse" => "ref",
        "matche" => "match",
        "om" => "if",
        "ellers" => "else",
        "selv" => "self",
        "la" => "let",
        "statisk" => "static",
        "struktur" => "struct",
        "forvent" => "expect",
        "mens" => "while",
        "bruk" => "use",
        "til" => "into",
        "sant" => "true",
        "oppregning" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rost(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
