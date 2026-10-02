# ADR-213 — Accélérer : tolérance, plafond, rituel allégé, bancs courts

- **Statut : actée**, S443, 2026-10-02 ; **décision de l'utilisateur** : *« J'aimerais que tu m'explique où cela bloque car tout
  prend du temps »*, puis, sur les quatre propositions, *« Ok go »*.
- **Amende** [ADR-187](ADR-187-methode-refondue-s321.md) (la méthode refondue) sur trois points — le rituel, l'arrêt sur un critère
  manqué, la durée d'un fil — et [METHODE](../../notes/METHODE.md) (les bancs).

## 1. Ce qui est constaté

De S434 à S442, neuf sessions : A324 corrigée, C7d-3b reçu, A322 levée, la bande sous Lax-Wendroff ; C7d-3a non reçu. Ce qui a
coûté : **A320**, cinq sessions sans cause trouvée, et elle verrouillait C7d-3a, puis C7d-3c, C7d-3d et la bascule des défauts ;
**deux arrêts pour des écarts infimes** (C7d-3b manqué de 0,01 % ; A322 sur la part relative d'un champ presque éteint), une session
de plus chacun, tranchés par l'utilisateur en une phrase ; **le rituel**, qui touche à chaque session la preuve, trois ou quatre
registres, le journal et REPRISE — environ un quart du temps ; **des calculs de une à deux heures** (la référence CPU à 12,5 cm).

## 2. Décisions

**D1 — Tolérance permanente.** Un critère **manqué de moins de 5 %** de son seuil, ou dont l'écart ne porte que sur **un champ
négligeable** (amplitude sous un dixième du phénomène mesuré, ou sous la tolérance d'usage — 3 mm d'image, S201), est **accepté sans
consulter l'utilisateur**. La preuve le dit : *« accepté, ADR-213 D1 »*, avec l'écart ; l'utilisateur peut revenir dessus. **Ne
s'applique pas** aux garanties de fonctionnement : un point fixe au bit, la production au bit, une conservation exacte, un refus
atomique — elles ne se négocient pas (METHODE, « Précision rapportée à l'usage »).

**D2 — Plafond des problèmes durs.** Un problème dont la cause n'est pas trouvée en **deux sessions** s'arrête : sa limite mesurée
(enveloppe, régime où il se montre, ce qui est écarté) s'écrit dans sa preuve et dans son angle mort, et **rien en aval ne l'attend**,
sauf décision de l'utilisateur. Il reste ouvert, avec un déclencheur. A320 est le premier cas.

**D3 — Rituel allégé.** À chaque session : EN-COURS, la preuve, le journal, REPRISE, `etat_projet.py --check`. **Les registres** —
feuille de route, liste, file active, index, notes datées d'angles morts — **se mettent à jour par lots de trois sessions** : la
troisième session depuis le dernier lot les fait, pour tout ce qui a changé depuis. Plus tôt seulement pour **une décision de
l'utilisateur** (dans la preuve et l'ADR qui la porte), **une porte reçue ou perdue**, ou **un angle mort nouveau de sévérité 3**.
REPRISE porte la ligne `Registres` : la session du dernier lot.

**D4 — Bancs courts.** Choisir le banc le plus court qui tranche : domaine réduit, maille plus grossière, durée minimale, calculs
en parallèle. **Plus de trente minutes de calcul** pour une mesure se justifient dans le plan, par ce qu'un banc plus court ne
trancherait pas.

## 3. Ce qui ne change pas

Le plan déclaré et commité seul, les critères écrits avant la mesure, un pas par commit, la preuve ouverte par « Reproduire », le
jeton et le battement, le non-poussage vers le dépôt distant. Les maillons (REPRISE §6) se comptent comme avant.
