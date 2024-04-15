use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_();

    let new_str = match ident_str.as_str() {
        "skibidi" => "std",
        "skibidont" => "no_std",
        "owo" => "core",
        "Roblox" => "Box",
        "linus_sebastian" => "drop",
        "forgor" => "forget",
        "alchemy" => "transmute",
        "npc" => "clone",
        "Cat" => "Copy",
        "cat" => "copy",
        "Based" => "Sized",
        "BlockChain" => "String",
        "nft" => "str",
        "as_nft" => "as_str",
        "to_nft" => "to_str",
        "to_blockchain" => "to_string",
        "MyGuys" => "Vec",
        "guys" => "vec",
        "4chan" => "thread",
        "4Chan" => "Thread",
        "farm" => "spawn",
        "Tweet" | "Repost" => "Send",
        "tweet" | "repost" => "send",
        "bwomp" => "process", // Blast Processing
        "head_out" => "exit",
        "Haxx" => "Command",
        "hax" => "command",
        "Pingas" => "HashMap",
        "KingdomCome" => "HashSet",
        "yall" => "collections",
        "Fortnite" => "Default",
        "fortnite" => "default",
        "Zoinks" => "Error",
        "Kinda" => "Option",
        "Yup" => "Some",
        "Nah" => "None",
        "Sussy" => "Result",
        "Vibin" => "Ok",
        "Trippin" => "Err",
        "NeverGonnaGiveYouUp" => "Infallible",
        "Sigma" => "Self",
        "sigma" => "self",
        "yappin" => "println",
        "yap" => "print",
        "rekt" => "break",
        "resonance_cascade" => "async",
        "hype" => "await",
        "♾️" => "loop",
        "dancin" => "move",
        "loot" => "crate",
        "simpin" => "unreachable_code",
        "dead_namin" => "as", // DID YOU JUST ASSUME MY TYPE??!!!
        "rock" => "const",
        "menace" => "static",
        "skill_issue" => "trait",
        "ohio" => "unsafe",
        "⤵️" => "in",
        "breed" => "from",
        "deploy_fazbear" => "dyn",
        "asmr" => "unwrap",
        "get_the_ref" => "as_ref",
        "come_and_go" => "io",
        "mainstream" => "extern",
        "fake" => "false",
        "fr" => "true",
        "nocap" => "assert",
        "nocap_same" => "assert_eq",
        "finna" => "fn",
        "giga" => "super",
        "intro" => "insert",
        "gyatt" => "get",
        "blyat" => "set",
        "legit" => "allow",
        "raid_shadow_legends" => "warn",
        "nu_uh" => "deny",
        "you_shall_not_pass" => "forbid",
        "panik" | "karen" => "panic",
        "pedo" => "mod", // Discord be like
        "gullible" => "mut",
        "oc" => "new",
        "imposter" => "where",
        "corn" => "for",
        "gyatt_amogus_rizz" => "get_or_insert_with",
        "chief" => "main",
        "viral" => "pub",
        "bruh" => None?,
        "yeet" => "return",
        "incel" => "impl",
        "pasta" => "ref",
        "lock_in" => "match",
        "vibe_check" => "if",
        "fanum_tax" => "else",
        "be" => "let",
        "jawline" => "struct",
        "spanish_inquisition" => "expect",
        "sleep_on" => "while",
        "uwu" => "use",
        "coming_inside" => "into", // 💀
        "coming_stepbro" => "into_iter", // 💀💀☠️
        "stepsis" => "iter",
        "mewing" => "enum",
        "Fellas" => "Group",
        "Snoopin" => "Ident",
        "GPTwitch" => "TokenStream",
        "GPTree" => "TokenTree",
        "wide_putin" => "span",
        "netflix" => "stream",
        "pushin_p" => "push",
        "clash_of_clans" => "extend",
        "non_mogger" => "delimiter",
        "Cologne" => "Punct",
        "LitReal" => "Literal",
        "minecraft" => "proc_macro",
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
pub fn brain_rost(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
