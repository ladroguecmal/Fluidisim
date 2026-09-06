# ADR-031 — Le front de mouillage élimine l'ordre 1, et une position de front n'existe pas sans seuil

- **Statut** : proposée
- **Session** : S23
- **Tranche** : ADR-007 §5.1 — candidats à évaluer pour δ (banc B3). Second critère d'entrée, après
  [`ADR-030`](ADR-030-l-equilibrage-est-un-critere-d-elimination.md).
- **Corrige** : rien. Complète `CAS-CANONIQUES` §C04, et **clôt l'action S22-4**.
- **Produit** : l'exécution de **C04** dans le mode `physics`, et la provenance de `H_SEC`

---

## 1. Ce que C04 a mesuré

Montage : canal plat sans frottement, `h₀ = 1 m` à gauche, **lit sec** à droite, lâcher à `t = 0`,
mesure à `t = 2 s`. Véhicule d'essai : le δ de S22, Saint-Venant 1D équilibré, `dx = 5 cm`.

| Grandeur | Mesuré | Ritter | Écart | Tolérance |
|---|---|---|---|---|
| `h` au droit du barrage | 0,4551 m | 0,4444 m | 2,40 % | 3 % |
| `u` au droit du barrage | 2,0302 m/s | 2,0881 m/s | 2,77 % | 3 % |
| **erreur L1 sur tout le domaine** | — | — | **0,84 %** | 3 % |
| **front, à `ε = 1 mm`** | 9,996 m | 11,934 m | **−16,24 %** | 3 % |

**Le solveur est bon partout et mauvais au front** : un facteur vingt entre l'erreur globale et
l'erreur locale. C'est exactement ce que C04 est fait pour attraper, et qu'aucune des trois autres
mesures ne révélait.

### 1.1 Les deux explications attendues sont fausses

Elles ont été testées, dans l'ordre où elles viennent à l'esprit.

**L'estimation des vitesses d'onde au lit sec.** Quand un côté d'une interface est sec, l'onde de
tête n'est pas `u ± c` mais l'invariant de Riemann `u + 2c` du côté mouillé (Toro). Prendre
`α = |u| + c` au contact du sec **borne la vitesse de propagation numérique en dessous de la vitesse
physique du front**. L'explication est correcte et documentée. Effet mesuré : **−16,09 % → −16,24 %**.
La formule juste est conservée — c'est la bonne physique, et elle vaudra dans d'autres
configurations — mais **elle ne répond pas de ce défaut**.

**Le seuil de séchage.** Voir §3 : six ordres de grandeur pour 0,25 point.

Reste, par élimination, la **diffusion numérique du schéma d'ordre 1 au voisinage du front**.

## 2. Décision : le front de mouillage est un second critère d'entrée à B3

Convergence de l'erreur de front, à `ε = 10⁻³`, sur cinq grilles :

```
nx =  200 → −21,11 %     ordre apparent, d'un raffinement au suivant :
nx =  400 → −19,34 %         0,13   0,27   0,36   0,41
nx =  800 → −16,09 %
nx = 1600 → −12,54 %     erreur L1 globale, elle, se comporte normalement
nx = 3200 →  −9,46 %
```

> **Décision. De même qu'ADR-030 élimine les candidats δ non équilibrés, un candidat qui n'est pas
> d'ordre supérieur *au front de mouillage* est éliminé avant d'entrer au banc B3.**
>
> L'exigence n'est pas « être d'ordre 2 en général » : beaucoup de schémas d'ordre élevé retombent à
> l'ordre 1 sur les cellules partiellement mouillées, précisément là où C04 mesure. **C'est la
> propriété au front qui est demandée, et C04 est ce qui la vérifie.**

**Le chiffre qui justifie « éliminé ».** À l'ordre apparent de 0,4, atteindre les 3 % de C04 depuis
`dx = 5 cm` demanderait `dx = 0,75 mm` — **×67 en résolution**, donc **×3·10⁵ en coût 2D** une fois
comptés les cellules (`dx⁻²`) et les pas de temps (`dx⁻¹`). C'est le pendant du chiffre de C01
(×10 500), **en trente fois pire**.

**Prudence sur l'exposant.** L'ordre apparent n'est pas stabilisé : il monte encore, de 0,13 à 0,41.
S'il tendait vers 1, le facteur tomberait à ×3 200 en 2D. **La conclusion est robuste dans les deux
cas** — elle reste éliminatoire — mais l'exposant est une estimation sur cinq grilles et doit être
cité comme telle. Le mesurer proprement demande le protocole de C08 (convergence sous raffinement),
qui n'est pas écrit.

## 3. Une position de front n'existe pas sans seuil, et le seuil peut renverser le verdict

C'est le résultat de méthode de la session, et il déborde largement C04.

La solution de Ritter tend vers zéro **continûment** : il n'existe aucune abscisse où l'eau
« commence ». Toute mesure de front est donc le lieu où `h` franchit un seuil `ε`, et ce seuil est
une convention. Sa position exacte, elle, se calcule :

```
h(x,t) = ε   ⇒   x = t·(2c₀ − 3√(g·ε))
```

| `ε` | position exacte du front | écart à `2c₀·t` |
|---|---|---|
| 10⁻⁶ | 12,510 m | −0,15 % |
| 10⁻⁴ | 12,340 m | −1,50 % |
| **10⁻³** | **11,934 m** | **−4,74 %** |
| 10⁻² | 10,649 m | **−15,00 %** |

**À `ε = 1 cm`, la convention déplace la référence de 15 % — cinq fois la tolérance de C04.** Un
harnais qui comparerait un front mesuré à seuil au front mathématique `2c₀·t` mesurerait donc
principalement sa propre convention. C'est ce que fait le témoin `C04-jet` : il affiche −20,21 % là
où la comparaison correcte donne −16,24 %. **Quatre points sur vingt sont de la pure définition.**

> **Règle. Une grandeur mesurée à seuil se compare à une référence prise au même seuil.** Le harnais
> le fait, et le témoin conserve l'écart entre les deux pour que l'ampleur de l'effet reste visible.

### 3.1 Pire : le seuil peut renverser le verdict

Le même solveur, à `nx = 3200`, affiche **−3,46 % à `ε = 10⁻²`** et **−10,83 % à `ε = 10⁻⁴`**. Le
premier est à un point de passer C04 ; le second échoue de trois fois la tolérance.

C'est la signature d'un front **étalé** : le profil numérique rejoint Ritter dans son corps et
traîne une queue mince. Mesurer haut sur le profil donne raison au solveur, mesurer bas lui donne
tort — et **l'énoncé de C04 ne dit pas lequel prendre**.

`ε = 10⁻³` a été retenu, avec cette justification : c'est le millimètre, la même unité que la
tolérance de C01, et il est trois ordres au-dessus de `H_SEC` donc insensible à lui. **C'est une
convention défendable, pas une valeur dérivée** — et à ce titre elle relève d'A106, dont elle est
maintenant la deuxième occurrence chiffrée.

## 4. `H_SEC` : provenance par mesure — clôture de l'action S22-4

S22 avait laissé `H_SEC = 10⁻⁶ m` posé au jugé, et l'avait relevé comme dette (action S22-4). C04
est le cas qui le met en jeu. Balayage sur six ordres de grandeur, sur la position du front :

```
h_sec = 10⁻⁹ → −16,38 %      10⁻⁷ → −16,31 %      10⁻⁶ → −16,24 %
h_sec = 10⁻⁴ → −16,07 %      10⁻³ → −16,13 %      volume = 20,000000 dans les cinq cas
```

**0,25 point d'effet sur seize.** La conclusion n'est pas celle qu'on cherchait :

> **`H_SEC` n'est pas un paramètre physique.** C'est un garde-fou contre une division par zéro, sa
> valeur est libre sur au moins six décades, et il n'a pas à recevoir la justification qu'I-14
> demande d'une grandeur physique.

**Sa provenance est cette mesure.** Une constante dont l'effet a été mesuré en a une, même quand
l'effet est nul — et c'est ce qui la sépare de `ρ_eau` (A103), qui déplace toute référence de
flottaison de 2,5 %. **I-14 n'est pas satisfait par un chiffre justifié, il l'est par un chiffre
dont on sait ce qu'il commande** ; les deux cas montrent que la réponse peut être « rien ».

## 5. Ce que C04 laisse ouvert

1. **C04 n'est pas passé, et ne le sera pas par ce véhicule.** Il reste rouge dans la batterie, ce
   qui est le comportement voulu : le solveur d'essai est d'ordre 1, et ADR-031 dit que l'ordre 1 ne
   passe pas. Un harnais qui masquerait cet échec masquerait la décision.
2. **L'ordre de convergence du front demande C08.** Voir §2. Cinq grilles ne suffisent pas à
   stabiliser un exposant qui bouge encore.
3. **Le seuil de mesure `ε` mérite d'être fixé par C04 lui-même**, comme C02 « produit `λ_cut` » :
   le seuil juste est celui au-dessus duquel le profil numérique n'est plus dominé par sa queue. Ce
   serait une grandeur dérivée du solveur, donc défendable au sens d'I-14, et non une convention.
   Angle mort **A110**.
4. **La friction de fond** reste absente (action S22-3). Sans effet sur C04, dont l'énoncé la
   supprime explicitement ; elle redevient un prérequis pour C03.
