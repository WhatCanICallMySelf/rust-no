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

    publik(låda) funktion kanske(i: u32) -> Kanske<Resultat<u32, Sträng>> {
        om i % 2 == 1 {
            om i == 42 {
                Någon(Fel(Sträng::från("merde")))
            } annars {
                Någon(Bra(33))
            }
        } annars {
            Ingenting
        }
    }

    asynkron funktion exempel() {
    }

    asynkron funktion exempel2() {
        exempel().vänta;
    }

    funktion huvud() {
        låt föränderlig x = 31;

        match x {
            42 => {
                skrivln!("omelette du fromage")
            }
            _ => skrivln!("voila")
        }

        för i av 0..10 {
            låt val = slinga {
                bryt i;
            };

            medan va x < val {
                x += 1;
            }

            x = om låt Någon(resultat) = kanske(i) {
                resultat.packa_upp()
            } annars {
                12
            };
        }

        //sekundär();
    }

    #[tillåt(onåbar_kod)]
    funktion sekundär() {
        fan!("skit också"); // for the true Swedish experience
        huvva!("men så obra!"); // for friends from up north
        oj!("de här sket sig"); // in SFW contexts
    }
}
