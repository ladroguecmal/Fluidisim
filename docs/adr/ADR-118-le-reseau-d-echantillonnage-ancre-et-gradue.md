# ADR-118 — Le réseau d'échantillonnage s'ancre sur ses frontières et gradue son pas

- Statut : actée, 2026-09-12, S187 ; autonomie technique S71.
- Applique SPEC-004 §6.1/§6.2, ADR-007 §5 ; mesures S170, S184, S186, S187.
- Ne remplace aucun ADR. Ne choisit ni solveur ni seuil de justesse.

## Contrat

Un consommateur volumétrique qui reconstruit la source `S` sur un réseau plus grossier
que ses mailles — décimation spatiale, SPEC-004 §6.2 — place ce réseau selon deux règles,
dans cet ordre de priorité.

**1. Ancrage.** Sur chaque axe, le premier et le dernier nœud se posent **sur** les
mailles extrêmes du domaine consommateur. Un réseau dont le dernier nœud tombe **hors**
du domaine fait interpoler la maille de bord sur toute la portée du réseau, et cette
maille de bord est précisément celle qui porte le maximum quand la source décroît en
profondeur. L'ancrage est **gratuit** — il ne change pas le nombre de nœuds — et il vaut
plus que toute graduation.

**2. Graduation.** Le pas du réseau suit la courbure du contenu :
`h(x) · √|∂²S/∂x²| = constante`, ce qui place les nœuds à incréments égaux de
`Φ = ∫ √|∂²S/∂x²| dx`. Cette règle s'applique par axe et n'a d'intérêt que sur un axe où
la courbure varie fortement — en pratique la verticale, parce qu'un mode profond décroît
en `exp(k z)`. Horizontalement, un pas uniforme suffit.

**3. Point d'arrêt.** Raffiner un axe cesse de payer dès que sa contribution passe sous
celle de l'axe le plus grossier : l'erreur totale sature alors sur cet autre axe. Le
dimensionnement s'arrête là, et les nœuds au-delà sont perdus.

Le réseau reste un réseau : le pas horizontal ne dépend pas de la profondeur. Un pas
horizontal variable ferait un arbre, l'interpolation trilinéaire ne s'y applique pas, et
rien ici ne le prescrit.

**Ce que le contrat ne dit pas.** Il ne fixe **aucun pas, aucun nombre de nœuds et aucune
erreur acceptable** : le seuil de justesse relève de B4 ou d'une réception perceptuelle,
et aucun n'est adopté (A50 reste ouverte sur ce point). Il dit où poser les nœuds qu'on a
décidé de payer, pas combien en payer.

## Pourquoi, et sur quels chiffres

[RESEAU-GRADUE-S187](../validation/RESEAU-GRADUE-S187.md), bloc 16³, `dx = 0,25 m`,
mailles intérieures à `z ∈ [−4,05 ; −0,80] m`, source `B + impacts + pression`, cadence
tenue à 1.

À nombre de nœuds verticaux égal et pas horizontal 2, l'erreur de champ passe de
**41,2 % à 6,8 %** à trois nœuds, de **13,6 % à 2,5 %** à cinq, et de **2,54 % à 1,69 %**
à huit, par le seul ancrage du dernier nœud. Facteurs **6,1 / 5,3 / 1,50**. La graduation
ajoutée par-dessus vaut **1,04 / 1,42 / 1,00** : réelle, modeste, utile entre quatre et
six nœuds.

Le mécanisme est mesuré, pas supposé : la métrique est un **maximum**, et
[COMPOSITION-ERREURS-S186](../validation/COMPOSITION-ERREURS-S186.md) §8.3 a montré que ce
maximum vit **intégralement** sur la tranche la plus haute du bloc. Un nœud posé sur cette
tranche supprime le terme dominant. La contre-épreuve horizontale le confirme par
l'absence d'effet : là où la source ne pique pas, l'ancrage vaut −2,5 %, −40 % et
**+1,3 %** selon le nombre de nœuds — non monotone, et sans le facteur six.

Ce n'est donc pas *« ancrer vaut mieux »* en général, mais **poser un nœud là où vit le
maximum de la métrique**. L'ancrage sur les frontières est la forme pratique de cette
règle pour une source qui décroît avec la profondeur.

La graduation est **dérivée** avant d'être codée (S187 §4) et elle bat deux témoins naïfs
à nœuds égaux : à 320 nœuds, 1,79 % contre 2,55 % pour un pas uniforme et 3,44 % pour un
pas géométrique de raison 2. Le profil qui la nourrit est **remesuré** par le programme
qui l'utilise, et il retombe sur la dérivation faite depuis les `k_eff` de S186 : rapport
extrême 12,63, pas vertical profond jusqu'à 3,55 fois celui du haut.

Gains à erreur égale, sur ce montage : **−37,5 % de nœuds** et −29 % d'erreur
simultanément contre l'isotrope `r = 2` ; **−78,4 % de nœuds** à erreur égale contre
l'isotrope `r = 4` ; et à nombre de nœuds identique (27), l'erreur divisée par **2,44**
contre l'isotrope `r = 8`.

## Ce qu'il faudrait pour l'inverser

- **Un contenu dont le maximum ne vit pas sur une frontière** du domaine échantillonné —
  une source piquée au centre du bloc, par exemple. L'ancrage n'aurait alors rien à
  supprimer, et la règle 1 deviendrait « poser un nœud sur la tranche du maximum », ce
  qu'elle est déjà au fond.
- **Une métrique qui ne serait pas un maximum.** Une norme quadratique répartirait
  l'erreur et l'ancrage y perdrait l'essentiel de son avantage. Le dépôt mesure en maximum
  depuis S170 ; changer de métrique est une décision qui n'est pas prise ici.
- **Un coût d'interpolation graduée qui mangerait le gain de nœuds.** Les poids d'un
  réseau gradué ne sont pas constants ; ce surcoût par maille n'est pas chiffré, et S184
  n'a mesuré que le réseau uniforme.
- **Une graduation à réactualiser.** Le profil est mesuré à un seul instant. Un contenu
  dont la structure verticale changerait vite demanderait de replacer les nœuds, et le
  coût de ce replacement n'est pas mesuré.

## Réception et limites

Protocole : [RESEAU-GRADUE-S187](../validation/RESEAU-GRADUE-S187.md). Six réceptions,
dont la reproduction en bits du chemin uniforme par le chemin général, et le contrôle
croisé qui redonne `max |S|`, `max |u'(T)|` et le plancher 0,386 % de S186. Empreinte
`0x6cf13183b4a240df`, `diff` strict identique sur deux exécutions. Le support historique
n'est pas modifié : `cadence_error` rend `0x39567a1d4bc2ba4c` et la sortie entière de
`composed_error` est inchangée. Workspace 331 réussis / cinq ignorés en debug et release.

Aucun code de bibliothèque n'est modifié : le réseau vit dans `examples/support/`, et ce
contrat s'adresse au consommateur qui reste à écrire. La loi de composition de S186 — le
maximum pour les modes causaux — a été établie sur le réseau **débordant** et n'est pas
rejouée sur un réseau ancré. Un seul montage, une seule profondeur de bloc, un seul
instant de profil. A50 et B4 restent partiels ; aucun solveur choisi.

## Suivi daté — S188, 2026-09-12 : une limite déclarée est levée

*Cette section n'amende pas le contrat ci-dessus et n'en corrige aucune erreur : elle lève une
limite que la réception déclarait, et elle est datée pour cette raison. Un ADR ne se réécrit
pas.*

La réception disait : *« La loi de composition de S186 — le maximum pour les modes causaux — a
été établie sur le réseau débordant et n'est pas rejouée sur un réseau ancré. »*
[COMPOSITION-ANCREE-S188](../validation/COMPOSITION-ANCREE-S188.md) l'a rejouée, à nombre de
nœuds identique et avec le même critère de jugement. **Le verdict est inchangé mode par mode**,
et mieux satisfait qu'en S186 : 0,895–1,060 contre 0,826–1,155 pour le maintien. La règle de
dimensionnement par parité des axes, que le contrat suppose, est donc valide **sur les deux
réseaux mesurés**.

Deux précisions que le rejeu ajoute au contrat sans le changer.

**La règle 3 — le point d'arrêt — se déplace avec l'ancrage.** L'erreur spatiale ancrée est
jusqu'à 3,7 fois plus faible à nombre de nœuds égal, donc la parité avec l'axe temporel arrive
plus tôt : à 125 nœuds, `c ≈ 6` au lieu de `c ≈ 20`. Le contrat dit de s'arrêter à la parité ;
il faut la recalculer sur le réseau qu'on emploie, et non reprendre un chiffre mesuré sur un
autre.

**La règle 1 — l'ancrage — a une raison plus précise que celle écrite ci-dessus.** Le contrat
dit de poser un nœud là où vit le maximum de la métrique. S188 mesure que ce maximum **ne
bouge pas** quand le réseau change : il vit sur la frontière haute parce que `|S|` y culmine,
et 39 cases jugées sur 39 le confirment. L'ancrage est donc sûr pour ce contenu, et la
condition de validité de la loi de composition qui en découle est écrite en **A232** : elle
demande que les maxima des deux erreurs coïncident. Le réseau **gradué** de la règle 2 est la
seule configuration connue susceptible de les séparer, et il n'est pas mesuré.
