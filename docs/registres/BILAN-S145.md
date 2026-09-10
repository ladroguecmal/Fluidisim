# Bilan d'avancement — S145, 2026-09-10

Refait à la méthode de [BILAN-S69](BILAN-S69.md), **76 sessions plus tard**, parce que celui-ci
oriente encore `REPRISE.md` §4 et qu'un état sans date se lit au présent alors qu'il ne l'est plus
(**A185**). Compteurs vérifiés à la source : fichiers du dépôt, sortie du harnais, `cargo test`.
Les colonnes « État » des tableaux d'actions antérieurs à S45 restent écartées, pour la même
raison qu'en S69.

## 1. Ce qui existe, et ce que cela pesait en S69

| | S69 | **S145** | facteur |
|---|---:|---:|---:|
| Sessions | 68 *(+5 réconciliées)* | **144** | ×2,1 |
| ADR | 52 | **98** | ×1,9 |
| Spécifications | 6 | **6** | — |
| Registres | 16 | **17** | — |
| Documents de validation | *non compté* | **73** | — |
| Angles morts | 188 | **210** | ×1,1 |
| Leçons | 184 | **227** | ×1,2 |
| Code Rust | 12 835 lignes | **34 128** *(76 fichiers)* | **×2,7** |
| Tests | 137 réussis, 5 ignorés | **275 réussis, 5 ignorés** | **×2,0** |
| Cas canoniques définis | 23 | **23** | — |
| Bancs définis | 11 | **11** | — |

**Un décompte recopié était faux, et c'est ce bilan qui l'attrape.** `README.md`, `docs/00_INDEX.md`
et `REPRISE.md` annonçaient **211 angles morts** depuis S142 : il y en a **210**, numérotés de A1 à
A210 sans trou. L'erreur a été introduite en S142 — A210 ajouté à un total de 209 a été écrit 211 —
puis recopiée par S143 et S144. C'est exactement le défaut que le rituel de fin de session
(`REPRISE.md` §6, point 5) demande de vérifier, et il a traversé trois rituels.

*Les sept « trous » apparents de la série des angles — A15, A19, A20, A27, A56, A57, A58 — sont des
artefacts de motif : ces sept-là vivent dans des tableaux, pas dans la liste à puces. S138 avait
déjà rencontré exactement ce piège.*

## 2. Construire le système — le bloc où S69 disait ~5 %

### 2.1 Où sont les 34 128 lignes

| ensemble | lignes | ce que c'est |
|---|---:|---|
| `water-core/src` hors essais | **15 523** | la bibliothèque |
| essais intégrés `tests_*.rs` | 2 998 | ce qui la reçoit |
| `water-core/examples` | 7 208 | **les sondes** — instruments de mesure de la conception |
| `water-harness` | 8 399 | H1 et H3 |

Les sondes pèsent presque la moitié de la bibliothèque. Ce n'est pas du gaspillage : c'est ce
qu'ADR-020 §*et une chose que S21 a apprise* annonçait — le code comme instrument de mesure de la
conception. Six des sept dernières sessions ont trouvé leur résultat dans une sonde.

### 2.2 Par couche, à la source

| couche | lignes | état réel |
|---|---:|---|
| **`W` — impacts** | 4 573 | `wave_event`, `wave_journal`, `radial_impact`, `impact_generator`, `composition`, `mixed_water`, `prepared_water`, `live_snapshot` |
| **`W` — pressions** | 5 195 | source, journal, contrôleur, instantané, modal, spectral, borné, cuisson gaussienne |
| **`δ`** | 2 516 | `delta` (Saint-Venant 1D, véhicule d'essai) et `shallow` (premier candidat exécutable) |
| **`B`** | 810 | couche de fond minimale, plus la flottaison de C10 |
| **`V`** | **0** | rien |

### 2.3 Ce que S69 disait, et ce qui a changé

> « `δ`, `W`, `V` n'existent pas. Deux véhicules d'essai 1D et un milieu spectral, qui ne sont pas
> le système. »

**Pour `W`, cette phrase a cessé d'être vraie**, et aucune session ne l'a dit. Le dépôt contient
aujourd'hui, reçu par 275 essais :

- un **contrat de production** (`WaveEvent`, ADR-055) et un **journal rejouable** borné
  (ADR-056) ;
- deux **champs propagés** dont un candidat radial dispersif en eau profonde, avec domaine de
  validité déclaré, bornes nommées et pente physique dérivée ;
- un **générateur** qui dit ce qu'un objet entrant dans l'eau donne au modèle (ADR-092) ;
- une couche de **pression** complète — source, admission, publication cohérente, reprise ;
- la **composition B+W**, le **service vivant** `LiveWater` avec sauvegarde et restauration, et
  l'**admission incrémentale exacte**.

C'est la trajectoire d'ADR-054 — `WaveEvent` → journal rejouable → impact propagé → … → B2 —
parcourue jusqu'à l'avant-dernière étape.

**Ce qui manque à `W` pour être la couche du jeu**, et qui n'a pas bougé :
1. **le sillage** — un objet en mouvement n'engendre rien ; seuls les impacts et les pressions
   existent. C'est l'étape *sillage/intégration* d'ADR-054, jamais commencée ;
2. **la sélection technologique**, c'est-à-dire **B2**, qui n'a jamais été exécuté. Le candidat
   radial reste un *candidat* : rien ne dit qu'il est le `W` du jeu ;
3. **l'anisotropie**, refusée à la construction depuis ADR-060.

**Pour `δ`**, la phrase de S69 tient : deux solveurs 1D, aucun domaine 2D, et le choix appartient
à B3.
**Pour `V`**, elle tient entièrement : zéro ligne.

## 3. Savoir mesurer — le bloc où S69 disait ~35 %

| | S69 | **S145** |
|---|---|---|
| Étages du harnais (SPEC-003 §10 en définit 6) | H1, H3 | H1, H3 — **et les pièces de H4** |
| Cas exécutés par le mode `physics` | 12 | **13**, dont un échec |
| Bancs exécutés sur 11 définis | **0** | **0** |

**L'échec est le même qu'en S69** : C04, front à ε = 1 mm, 16,2 % d'écart pour 3 % de tolérance.
Les trois autres grandeurs de C04 passent. C'est un désaccord de définition du front, documenté,
pas une régression.

**Les pièces de H4 existent sans que l'étage ait été déclaré.** SPEC-003 le définit comme « oracle
lent 1D/2D, convergence, protocole iso-qualité ». Le harnais contient `oracle.rs` — l'oracle croisé
de S37, qui confronte `delta.rs` et `shallow.rs` champ à champ — et `rapport_convergence.rs`, et
C08 mesure la convergence sous raffinement. Ce qui manque est le protocole iso-qualité et le
2D. **Dire que le harnais a deux étages sur six est donc devenu inexact par défaut** ; dire qu'il
en a trois serait inexact par excès.

**Le vrai chiffre de ce bloc n'a pas bougé : zéro banc sur onze.** C'est ce que S69 appelait le
goulot, et soixante-seize sessions plus tard il est intact.

### Ce qui a grandi, et qui n'est pas dans ce tableau

La couche `W` est reçue par **275 essais intégrés** et **73 documents de validation** — dont
`ENVELOPPE-PRESSION`, `PENTE-REELLE`, `MIGRATION-PENTE`, `TRANSPORT-ETENDU`, `BILAN-CANDIDAT-ETENDU`.
Aucun de ces reçus ne passe par le harnais.

Ce n'est pas une faute : les essais intégrés vérifient ce qu'un scénario ne peut pas voir — bits
publiés, atomicité des refus, bornes nommées. Mais cela veut dire que **le dispositif de mesure
prévu par SPEC-003 n'a pas suivi la couche qui a été écrite**, et que les bancs, qui sont la
raison d'être des étages H3 à H6, restent hors de portée pour une raison qui n'a rien à voir avec
`W` : ils demandent des étages qui n'existent pas.
