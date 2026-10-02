# ADR-198 — La voie d'A289 : δ relatif à la dynamique de B

- **Statut : actée**, S369, 2026-09-26, par le projet, sous la délégation de l'utilisateur
  ([ADR-197](ADR-197-reponses-du-2026-09-26.md) D6) : *« le choix le plus favorable au réalisme ainsi que les
  performances, simple »*.
- **Tranche** A289 ([angle mort](../registres/ANGLES-MORTS.md)), que S319 avait laissée entre trois voies — rappel lent,
  durée de vie bornée, dispersion d'amplitude dans B — et **en prend une quatrième**.
- **Précise** le complément S178 de [SPEC-004](../specs/SPEC-004-interfaces.md) (« le résidu continu S, à soustraire ») :
  pour δ couplé, ce résidu n'est plus une source. [ADR-114](ADR-114-source-continue-du-fond-profond.md) n'est pas réécrit.
- Preuve : [MER-S369](../validation/MER-S369.md).

## 1. Le constat

Le pas couplé de S297 résout la surface totale et donne à δ, comme sources, trois termes qui ne dépendent que de B : son
résidu de quantité de mouvement, son transport entre le plan moyen et sa propre surface, son erreur de pression à cette
surface. B linéaire ne satisfait pas les équations complètes ; ces restes nourrissent δ même quand rien ne le perturbe —
**c'est la croissance de S319**. Retirés, δ nul reste nul au bit sous la houle (preuve §1).

## 2. Décisions

**D1 — δ est relatif à la dynamique de B.** Les trois termes propres à B ne sont plus des sources de δ
(`Volume3::set_relative_background(RELATIVE_ALL)`) ; restent les termes croisés et ceux de δ seul. C'est le mode du pas
couplé pour l'ordre E et pour la production. **Le défaut du cœur reste le pas de S297** tant que la production GPU
(`delta3d_step.wgsl`) ne porte pas le même mode au bit — les deux basculent ensemble ; le pas de S297 reste le témoin des
mesures de S319 et S322.

**D2 — Pourquoi elle, sous les trois critères.** *Réalisme* : la mer de fond, dans le domaine, est B comme partout
ailleurs — aucune couture, cohérence de phase exacte par construction (4.21) ; aucune perturbation amortie, aucun domaine
recréé. *Performances* : trois termes de moins ; sous B seul, la pression n'a plus rien à résoudre (E1 en 51 s au lieu
de ≈ 170). *Simplicité* : trois retraits locaux ; B inchangé, donc la mer de tous inchangée. Les trois voies d'A289
échouaient chacune sur un critère : le rappel lent devait amortir toute perturbation plus vite que 0,1 s⁻¹, la durée de
vie bornée recréer les domaines avant dix secondes, la dispersion d'amplitude corrigeait la vitesse de phase sans les
harmoniques liés — et changeait B pour tous.

**D3 — Ce qu'elle abandonne.** Les corrections non linéaires de B à lui-même dans le domaine (crêtes de Stokes), que B
n'a nulle part ailleurs. Rendre B plus réaliste reste possible et resterait cohérent avec δ.

**D4 — L'ordre E reste bloqué, par une autre cause, nommée : A320.** Derrière la source, une instabilité convective des
perturbations sous houle raide, portée par le terme de cisaillement `u'·∇U` (preuve §3) ; son taux baisse à maille fine.
Suite, sous la même délégation : les termes croisés sous la forme de Bernoulli, `∇(U·u')`, exacte pour un fond et une
perturbation irrotationnels, gratuite ; si elle ne suffit pas, les trois critères départagent à nouveau, sur mesure.

**D5 — Le critère de restitution de l'ordre E se refond** avant d'être rejoué : dans une mer, le transport croisé B×δ à
la ligne couvre vingt fois le volume net d'un paquet presque de moyenne nulle (preuve §4). Le nouveau critère s'écrit
avant la mesure, sur la durée ou sur le transport propre de δ.

## 3. Ce qu'elle ne fait pas

Elle ne change ni B, ni W, ni aucun invariant ; elle ne lève pas le blocage de l'ordre E (D4) ; elle ne touche pas la
production GPU, dont la bascule est un travail daté.

*Note du 2026-10-02, S443* : **le mode relatif devient le défaut** de `Volume3` et de `Step3` (C7d-3b, décision de l'utilisateur :
*« J'accepte ta proposition »*), avec le fantôme latéral d'A324 et la bande sous Lax-Wendroff d'A322 ; le pas de S297 reste
atteignable (`set_relative_background(0)`). [Preuve](../validation/APIC-CARTE-S416.md) §22.11.

