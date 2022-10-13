# rost

![](https://github.com/vojd/rost/raw/huvud/logo.jpeg)

Aren't you _uttråkad_ from writing Rust programs in English? Do you like saying
"fan" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Swedish touch to your
programs?

**rost** (Swedish for _Rust_) is here to save your day, as it allows you to
write Rust programs in Swedish, using Swedish keywords, Swedish function names,
Swedish idioms.

This has been designed to be used as the official programming language to
develop the future Swedish sovereign operating system. 

If you're from Sweden's or any other government with Swedish as an official 
language: [bnjbvr](https://github.com/bnjbvr) will be awaiting your donations on
[liberapay](https://liberapay.com/bnjbvr/).
(Vänligen notera att [bnjbvr](https://github.com/bnjbvr) skrev den ursprungliga Franska versionen och förtjänar därför världens tack)

You're from Finland (or elsewhere) and don't feel at ease using only Swedish words? 

Don't worry!
Swedish Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with Rost:

### trait and impl (aka konvention och implementera)
 
```rust
rost::rost! {
    extern låda rost;

    använd std::collections::Ordbok som Bok;

    konvention NyckelVärde {
        funktion skriv(&själv, nyckel: Sträng, värde: Sträng);
        funktion läs(&själv, nyckel: Sträng) -> Resultat<Kanske<&Sträng>, Sträng>;
    }

    statisk föränderlig ORDBOK: Kanske<Bok<Sträng, Sträng>> = Ingenting;

    struktur Konkret;

    implementera NyckelVärde för Konkret {
        funktion skriv(&själv, nyckel: Sträng, värde: Sträng) {
            låt bok = osäker {
                ORDBOK.ta_eller_för_in_med(Standard::standard)
            };
            bok.för_in(nyckel, värde);
        }
        funktion läs(&själv, nyckel: Sträng) -> Resultat<Kanske<&Sträng>, Sträng> {
            om låt Någon(bok) = osäker { ORDBOK.som_ref() } {
                Bra(bok.läs(&nyckel))
            } annars {
                Fel("hämta ord".till())
            }
        }
    }
}
```

### Support for regional languages

```rust
#[tillåt(onåbar_kod)]
funktion sekundär() {
  fan!("skit också"); // for the true Swedish experience
  huvva!("men så obra!"); // for friends from up north
  oj!("de här sket sig"); // in SFW contexts
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Sådär! Det var det.

## Bidrag / contributions

First of all, _tack så mycket_ for considering participating in this joke, the
Swedish government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `huvud` (Swedish for
`main`) branch.

Please don't introduce swear words, though: we will not excuse your Swedish.

## But varför would you do that?

- Det är ball
- We must not let the Finns and Danes down 
- To tease the Norwegians who don't have a localized Rust (yet)
- To learn more about proc_macros
- Det är verkligen ball

## Other languages

- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [Ржавый](https://github.com/Sanceilaks/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)
- Thai: [sanim (สนิม)](https://github.com/korewaChino/sanim)
- Swiss: [roeschti](https://github.com/Georg-code/roeschti)

## Ett stort tack till

- [bnjbvr](https://github.com/bnjbvr) for making the original French version on which this is based

## Licensen

[License Publique Rien à Branler](http://sam.zoy.org/lprab/),
the official translation of the [WTFPL](http://www.wtfpl.net/)
by the same author.

(Ok, den är på Franska men det går ändå)
