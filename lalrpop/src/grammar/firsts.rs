use intern;
use grammar::{parse_tree, repr};
use std::collections::{HashMap, HashSet};

pub struct FirstSet {
    pub set: HashSet<parse_tree::TerminalString>,
    pub epsilon: bool
}

pub fn firsts(grammar: &repr::Grammar) -> HashMap<parse_tree::NonterminalString,
                                                  FirstSet> {
    let mut ret = HashMap::new();
    for &nt in grammar.nonterminals.keys() {
        ret.insert(nt, FirstSet {
            set: HashSet::new(),
            epsilon: false
        });
    }

    // fix point search
    let mut fp = false;
    while !fp {
        fp = true;

        for (nt, data) in grammar.nonterminals.iter() {
            'a: for prod in data.productions.iter() {
                for sym in prod.symbols.iter() {
                    match sym {
                        // if we encounter an opaque symbol (i.e. either a
                        // terminal of a non-terminal that does not derive
                        // epsilon, we just stop processing this rule. This
                        // is done with "continue 'a"
                        &repr::Symbol::Terminal(t) => {
                            fp &= !ret.get_mut(nt).unwrap().set.insert(t);
                            continue 'a
                        }

                        &repr::Symbol::Nonterminal(sym) => {
                            // trick to be able to access mutably ret[idx]
                            // while ret[sym] is immutabily borrowed. this
                            // is safe since the case sym == idx doesn't
                            // interest us anyway...
                            //let ((sub1, sub2), idx1, idx2) =
                            //    if sym > idx {
                            //        let (fst, snd) = ret.split_at_mut(sym);
                            //        ((snd, fst), 0, idx)
                            //    } else if sym < idx {
                            //        (ret.split_at_mut(idx), sym, 0)
                            //    } else { continue };
                            let clone = ret[&sym].set.clone();

                            for s in clone.iter() {
                                fp &= !ret.get_mut(nt).unwrap().set.insert(s.clone());
                            }

                            if !ret[&sym].epsilon { continue 'a }
                        }
                    }
                }

                // if we arrive here without jumping to the next
                // iteration of 'a, it means all the symbols in
                // this production devive epsilon, or there are
                // no symbols (i.e. this is a e-rule) so:
                fp &= ret[nt].epsilon;
                ret.get_mut(nt).unwrap().epsilon = true;
            }
        }
    }

    ret
}

pub fn compute(grammar: &repr::Grammar, nts: &[String]) {
    let firsts = firsts(grammar);
    for nt in nts {
        println!("=== FIRST({}) ===", nt);
        for sym in firsts[&parse_tree::NonterminalString(intern::intern(&nt))].set.iter() {
            println!("{}", sym)
        }
    }
}
