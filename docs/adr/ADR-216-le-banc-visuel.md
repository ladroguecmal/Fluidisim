# ADR-216 — Le banc visuel : mesurer plutôt que regarder

- **Statut : actée**, S471, 2026-10-04 ; **décision de l'utilisateur** : après R39 (*« Tous les verdicts sont validés, mais pas
  définitifs, car toujours peaufinables »*), sur la question *« réfléchir à la façon de créer du code de jeu visuel ; pour analyser des
  images tu regardes des pixels, peut-être qu'il y aurait une meilleure façon de travailler avec toi, et sans forcément moi, en pure
  autonomie »*, le banc proposé est reçu : *« Je valide ce banc »* ; avec six vidéos de référence
  ([REFERENCES-VIDEO-S471](../validation/REFERENCES-VIDEO-S471.md)).
- **Complète** [ADR-215](ADR-215-autonomie-jusqu-a-une-v1-solide.md) (l'autonomie : D2 laissait à l'utilisateur le jugement visuel) et
  l'outil `cible_image.py` (S308 : des grandeurs comparables entre une photographie et un rendu) ; **ne change pas** la méthode.

## 1. Le constat

Ce que l'agent voit d'une image : l'image entière, réduite — la composition, une dominante, un défaut grossier. Ce qu'il voit mal : le
réalisme fin, le détail, et **le mouvement** (il ne voit que des images fixes) ; et il peut se croire sûr à tort. Ce qui a marché
(S468–S470) : des **masques de contrôle** (une grandeur en fausses couleurs) et des **comparaisons au bit** — un signal net, pas une
impression. « Crédible » ne se mesure pas ; ses composantes, si.

## 2. Décisions

**D1 — Des grandeurs comparables sans connaître la prise de vue**, par zone (un rectangle en fractions du cadre), rapportées à la
médiane de la zone : la répartition de la luminance (p05 … p99 ÷ p50), le contraste local, la teinte des creux et des crêtes (B/G,
B/R), les clairs, l'écume, la part haute fréquence, l'anisotropie (de `cible_image.py`) ; et **le temps** : l'énergie de mouvement par
seconde, la période dominante et sa netteté, le renouvellement des clairs (la durée de vie des reflets), les coupes d'un montage
écartées. Le détail : `outils/banc_visuel.py` (en-tête).

**D2 — Les références se mesurent là où elles se lisent.** Une vidéo fournie par l'utilisateur est mesurée dans la page du navigateur
qui la lit (`outils/banc_visuel.js` : l'image de la vidéo lue dans un canevas). **Seuls des nombres entrent dans le dépôt** — ni image
ni vidéo (droits d'auteur ; SPEC-005 §11.3, rien de binaire), l'adresse, le propos de l'utilisateur et les zones.

**D3 — Un seul calcul, deux langages.** `banc_visuel.py` (nos rendus) et `banc_visuel.js` (les références) font le même calcul ;
`banc_visuel.py --egalite` les confronte sur les mêmes images (critère 1 % ; S471 : écart nul à quatre chiffres).

**D4 — Des scènes miroirs.** Pour chaque référence, la scène de Godot qui s'en approche le plus (le point de vue, la hauteur, l'état
de la mer), mesurée sur les mêmes zones ; ce qu'aucune scène ne peut encore montrer (le déferlement, l'écume, les rochers, la plage)
est inscrit comme tel, avec ce qu'il attend.

**D5 — Les verdicts deviennent des images de non-régression.** Une image reçue par l'utilisateur est gardée (localement) avec ses
mesures ; une modification qui la change au-delà d'une tolérance dite se signale d'elle-même.

**D6 — Les remarques de l'utilisateur deviennent des critères.** « Trop clair », « pas crédible », « l'écume est fausse » : chaque
remarque est traduite en une grandeur et un écart toléré à une référence, inscrits ; le jugement s'accumule en une spécification
visuelle vérifiable sans lui.

**D7 — Les vues de contrôle et les sondes.** Dans Godot, une grandeur à la fois en fausses couleurs (ombre, profondeur, normales,
caustiques, mouillure) et des sondes chiffrées en des points fixes du monde : le diagnostic d'un défaut se lit sans l'interpréter.

**D8 — À traitement égal (S473).** Une référence publiée est compressée ; la compression lisse le scintillement fin de l'eau (le
mouvement mesuré d'une même séquence varie de 0,28 à 1,29 par seconde selon le codec et le débit, MIROIRS-S472 §S473). Avant de
comparer les grandeurs **fines et temporelles**, notre séquence passe par le codec et le débit de la référence (lus dans sa page),
dans le navigateur (`outils/banc_visuel_codec.js`) ; on rapporte les deux mesures, brute et compressée — l'encodeur de la référence
n'est pas le nôtre, la vérité est entre elles.

## 3. Conséquences

- L'utilisateur juge aux jalons ; entre eux, l'agent compare des nombres à des références qu'il a fournies.
- Les écarts mesurés commandent les sessions de rendu (ADR-191 : une de rendu, une de physique) au même titre que ses verdicts.
- Limites dites : une vidéo publiée est compressée, étalonnée, parfois retouchée ; sa balance des blancs et son exposition sont
  inconnues — d'où les seules grandeurs normalisées ; un écart de quelques pour cent n'est pas un défaut.
- Revenir dessus : l'utilisateur peut à tout moment juger une image contre les mesures ; son verdict passe avant.
