# ADR-271 — Le film du rivage appartient à Saint-Venant 2D

- **Statut : actée**, S679, 2026-10-07. Décision technique (ADR-215, ADR-222 : trancher seul, la méthode se révise elle-même).

## Contexte

APIC 3D ne tient pas l'eau au repos dans le film d'un rivage en pente lisse.

- [S640](../validation/FACES-COUPEES-APIC3D-S640.md) : 0,26 m/s.
- [S678](../validation/FILM-RIVAGE-S678.md) : deux causes, la surface du film et les faces à peine ouvertes. Leur remède tient quatre
  plages sur six.

Saint-Venant 2D porte le mouillage, le séchage, le ressaut et la remontée, validés de S613 à S628.

## Décision

**D1.** Le film du rivage, du ressaut formé après la plongée jusqu'au jet de rive, est porté par **Saint-Venant 2D**. APIC 3D garde
la bande où la surface se retourne. Le raccord est un plan fixe, au-delà du point de plongée et à au moins trois mailles de profondeur
au repos.

**D2.** Les deux côtés échangent **un seul flux par pas** (Riemann, HLL) : la masse au bit.

**D3.** Les remèdes de S678 (`film`, `seuil_face`) restent éteints dans APIC. Ils ne sont pas retirés : ils resservent si une plage sans
relais doit être tenue.

## Conséquences

- La campagne est conçue dans [RELAIS-RIVAGE-S679](../registres/RELAIS-RIVAGE-S679.md), en cinq étapes.
- 4.14 « une surface fiable en eau mince au rivage au repos » se jugera sur le relais (étape 1), et non sur APIC seul.
