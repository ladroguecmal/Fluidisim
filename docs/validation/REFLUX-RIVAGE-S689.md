# Le reflux à travers le raccord au rivage ; la dette remboursée — S689 (liste 4.14)

*S689, 2026-10-08, en autonomie, vers la v2.* L'étape 3 du relais au rivage ([conception](../registres/RELAIS-RIVAGE-S679.md)) : l'eau
qui redescend la plage repasse de Saint-Venant 2D dans APIC 3D.

## Reproduire

- `cargo test --release --offline -p water-core --lib the_backwash_crosses_the_shore_relay_s689 -- --ignored --nocapture`, depuis
  `code/water-core` (≈ 5 min ; depuis une copie du binaire).

## Ce qui a été fait

**La dette remboursée au quantum.** Jusqu'ici (S685), la dette s'accumulait : c'est ce que Saint-Venant a pris, moins ce qu'APIC a
laissé sortir, et elle atteignait 2 à 3 quanta par rangée en 3 s. Désormais :

- quand elle atteint un quantum, APIC rend la particule de sa dernière colonne la plus proche du bord (`take_right`) ;
- quand elle descend sous moins un quantum, Saint-Venant reçoit ce volume dans sa première maille.

Les remboursements sont comptés (`rendues`, `ajoutes`).

## Mesuré

**Le montage** : l'onde de S685 suivie 10 s à 5 cm. Elle monte, redescend, repasse dans la 3D, se réfléchit au mur du large et revient.

| critère (écrit avant) | mesuré |
|---|---|
| (1) la masse à 10⁻¹² près | **2,8·10⁻¹⁶** |
| (2) la dette sous un quantum par rangée | **0,998** au plus |
| (3) la dernière colonne sous 10 par maille mouillée | **9,20** |
| (4) la vitesse des particules sous 1 m/s | **0,539 m/s** |

**Les volumes échangés sur 10 s :**

| sens | comment | volume |
|---|---|---|
| de la 3D vers Saint-Venant | particules qui franchissent le bord | 5,28 L |
| | particules rendues (le remboursement) | 20,14 L (1 289) |
| de Saint-Venant vers la 3D | particules posées | 25,84 L (1 652, aucune refusée) |
| | quanta rendus à Saint-Venant | 1,42 L (91) |

**Ce qui se voit.** La sortie de la 3D passe surtout par le remboursement (79 %), non par les particules qui franchissent le bord. Le
flux de Saint-Venant demande au bord de la 3D une vitesse `F/h`, mais les particules proches du bord n'avancent qu'à la vitesse
interpolée de la grille, plus lente. Le remboursement retire alors la particule la plus proche du bord : c'est une sortie au même
endroit, au quantum près. Le reflux, lui, entre par les particules posées.

## Ce que cela dit

Le relais tient un aller-retour complet et ses réflexions sur 10 s : la masse au bit, la dette bornée, la colonne du bord ni vidée ni
tassée, aucune vitesse parasite. Le reflux traverse le raccord dans les deux sens.

**Ne fait pas** : la sortie par advection seule (la vitesse des particules au bord) ; un état extérieur par rangée. L'étape 4, la
vague qui plonge avec les deux raccords, suit.
