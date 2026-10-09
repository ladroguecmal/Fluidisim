# ADR-285 — Cinquantième revue de méthode (S726–S730)

- **Statut : actée**, S731, 2026-10-09 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-283](ADR-283-quarante-neuvieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S726 | la revue (ADR-283) | — | — |
| S727–S729 | B4a, B4b, B5 : chaque critère tenu au premier essai ; deux erreurs de compilation (un nom de méthode déjà pris, `as f64 <` lu comme un générique) | aucun | aucune |
| S717–S730 | **un témoin qui portait un raccord non mesuré.** Le « tout-3D » de la plage gardait Saint-Venant au rivage à 10,775 m, là où le jet retombe. Son mur montait à 28 cm (S730). Ses remontées servaient de juge, et la conclusion de S718–S719 (« le large SGN en cause », +3,3 cm) en dépendait. Avec le raccord au-delà du jet, l'écart devient −1,8 cm. **C'est l'utilisateur qui l'a vu, sur une image de R43** | une conclusion fausse en partie pendant treize sessions ; une note datée sur ADR-278 | **D1** |
| S718 | **une attribution sans faire varier ce que le témoin et l'objet partagent** : l'écart de remontée a été attribué à SGN après une seule variation (avec ou sans la mort). Le raccord du rivage, commun aux deux montages, n'a pas varié | la même conclusion | **D1** |
| S730 | **le filtre d'`essai.py`, une sous-chaîne** : E1 a lancé E2 aussi (son nom contient celui de E1), et E2 d'abord | ≈ 20 min de calcul dans un ordre non prévu, sans perte | **D2**, fait |
| S730 | **un ADR créé sans sa ligne d'index** : `fermer.py` a refusé la fermeture (à raison), et l'arbre est resté modifié par le script de fin, restauré à la main avant de relancer | quelques minutes | **D3** |
| S731 | le piège d'ADR-223 D4 retrouvé : un `\n` dans une chaîne Python d'un *heredoc*, devenu un vrai saut de ligne | une correction | aucune : la règle existe |

## 2. Décisions

**D1 — Un témoin se contrôle comme l'objet qu'il juge.**
- Toute frontière qui reste dans une référence (un raccord, un mur, une entrée) a son instrument de saut (ADR-284 D4), lu sur toute la durée.
- La référence est **regardée** à ses instants critiques avant la première conclusion tirée contre elle : une image du déferlement, du jet,
  de la remontée.
- **Un écart attribué à un composant se vérifie en faisant varier ce que le témoin et l'objet ont en commun** : un raccord partagé, un mur
  partagé. Une variation qui laisse identique une partie commune ne l'innocente pas. C'est le complément d'ADR-256 D2 (l'amplitude, la
  maille).

**D2 — `essai.py` ne lance que ce que le nom désigne exactement** (fait en S731).
- Un nom qui désigne plusieurs essais est refusé, sauf si l'un porte exactement ce nom : lui seul tourne, avec `--exact`.
- `--plusieurs` accepte les lancer tous ; `--liste` montre ce qui tournerait.

**D3 — Un ADR naît avec sa ligne d'index**, dans le commit qui le crée. Si `fermer.py` refuse, l'arbre est restauré (`git restore`) avant de
relancer la fermeture : le script de fin s'applique à un arbre propre.

**D4 — La demande de l'utilisateur, le 2026-10-09** : *« ce système qui choisit quel type de simulation doit être le plus peaufiné et solide,
car c'est grâce à lui que l'on aura le meilleur compromis entre réalisme et performance »*. **Le sélecteur des domaines** (ADR-284 : où vont
SGN, la 3D et Saint-Venant, et quand) devient la campagne suivante, avant la bande étroite. Sa conception (S732) suit trois règles :
- **une batterie de scènes**, non une seule : un déferlement plongeant au large et près du bord, un déferlement glissant, une vague qui ne
  déferle pas, un corps qui entre dans l'eau. Chacune a un témoin tout-3D dont les frontières restantes sont mesurées (D1) ;
- **trois juges par scène** :
  - le réalisme : le déferlement dans la tolérance d'ADR-278 D2, le mur sous 1 cm, la remontée ;
  - le coût : le rapport au tout-3D ;
  - la solidité, détaillée ci-dessous ;
- **la solidité** :
  - aucun clignotement : les bascules comptées par domaine, sous l'hystérésis ;
  - une prédiction fausse, injectée exprès, rattrapée sans défaut visible ;
  - le repli prudent : dans le doute, la 3D plus large ;
  - chaque décision du sélecteur journalisée et rejouable.

## 3. La prochaine revue

S736.
