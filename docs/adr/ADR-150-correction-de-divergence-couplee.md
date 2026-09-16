# ADR-150 — Corriger le défaut de divergence sur la vitesse couplée

Actée S251, 2026-09-16, autonomie S71 ; traite A283. Complète ADR-149, conserve
les tolérances d'ADR-143/144 et le repli multigrille ADR-147.

Le pas couplé essaie d'abord les projections reçues. Si elles déclarent un plancher
avec divergence refusée, il applique **une** projection supplémentaire au champ
de vitesse déjà corrigé : `L q = -(rho/dt) div(v_corrigé)`, couvercle homogène,
puis `v_final = v_corrigé - (dt/rho) grad(q)`. Pression publiée : somme f32 `p+q`.
Ne pas reconstruire ensuite v_final depuis cette somme arrondie : cela réintroduirait
l'annulation que l'on corrige. Un tampon de pression f32, 4 octets par maille, est
réservé à l'initialisation ; pas d'allocation pendant le pas.

Ce n'est ni une force nouvelle ni une modification du résidu continu B/W. Il s'agit
d'un affinage de la projection discrète. Le second solveur garde ses critères propres
et mesure la divergence du champ final. Si elle reste refusée, le pas reste refusé.
Pas d'affinage au seul plafond d'itérations sans plancher, ni de tentative sans fin.
Chaque projection garde le plafond fourni ; toutes partagent le même budget mural.

`Report.refinements` signale cet affinage (0 ou 1). `iterations` cumule le travail
des projections du pas couplé ; residual/floor/backward_error décrivent **le dernier
système résolu**, q en cas d'affinage, et divergence/divergence_plain le champ final.
Les chemins non couplés ne changent pas ; un pas couplé déjà reçu n'est pas affiné.
Expiration et refus restaurent u/w/p. Le mode de couvercle homogène est restauré
même à l'expiration ; eta et ses restes ne sont pas modifiés.

[Diagnostic et réception](../validation/DEMARRAGE-PLAT-S251.md). Pas de réception
universelle de la pression, de la surface mobile couplée ni du budget I-05.
