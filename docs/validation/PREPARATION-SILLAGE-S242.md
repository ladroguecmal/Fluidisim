# La préparation CPU du sillage, par image — S242, 2026-09-15

Traite le poste devenu dominant du budget de l'hôte. Depuis le LOD spatial de
[S234](LOD-SILLAGE-S234.md), le GPU eau vaut **0,44 ms** par image et la seule préparation CPU du
sillage **3,17 ms de médiane pour 13,2 ms de maximum** ([S240](ALLOCATIONS-HOTE-S240.md) §2.2), quand
[ADR-125](../adr/ADR-125-budget-image-60hz-deux-ms.md) donne **2 ms à toute l'eau**. La conclusion de
[S225](CADENCE-HOTE-S225.md) — « les techniques restantes sont du côté GPU » — n'est plus vraie.

## 1. Protocole, écrit avant mesure et construction

### 1.1 Ce que la lecture du code établit

`Timeline::render_components` fait, **à chaque image et pour chacun des 4 096 nœuds**, une boucle sur
les **24 segments** du journal — trois sillages de huit tronçons de 2 s — en testant
`mode.birth() < time && time < mode.forcing_end()`. Soit **98 304 tests par image**.

Or `Timeline::fold`, juste au-dessus, écrit et utilise le fait que **naissance et durée sont les
mêmes pour tous les nœuds** : il ne lit que la **rangée 0** pour connaître les fins de tronçon.
L'ensemble des segments actifs à un instant ne dépend donc **que du temps**, jamais du nœud.

Les huit tronçons d'un sillage se suivent sans recouvrement : à tout instant, **au plus un segment
par sillage** est en forçage — trois sur vingt-quatre —, et après 16 s de forçage, **aucun**.

### 1.2 Thèse

Un test dont le résultat est identique pour les 4 096 nœuds se calcule **une fois par image**. Hisser
cette sélection hors de la boucle des nœuds **ne change aucune opération flottante** : les mêmes
termes sont additionnés dans le même ordre, seuls les tests qui ne retenaient rien disparaissent. Le
gain est donc **gratuit au bit**, et il croît avec le nombre de tronçons de la scène.

### 1.3 L'instrument, et pourquoi il ne regarde pas dans la fonction

Instrumenter l'intérieur de `render_components` demanderait d'y poser des horloges, donc d'en
déplacer le coût. **On fait varier le paramètre au lieu d'ouvrir la boîte** : le même banc mesure la
préparation à **un, deux et trois sillages** — 8, 16 et 24 segments — pour un nombre de nœuds
**inchangé** (4 096). La pente contre le nombre de segments *est* la part de la boucle interne, et
elle se mesure avant comme après. Banc : `examples/sillage_troncons.rs`, fixture de S212/S235
inchangée.

Deux fenêtres, parce qu'elles n'exercent pas le même chemin :

- **pendant le forçage** (à partir de 3 s d'âge) : un segment actif par sillage ;
- **après le forçage** (à partir de 24 s) : **aucun** segment actif, tout est replié — c'est là que
  la boucle interne ne rapporte rien du tout, et que le coût inutile est le plus visible.

### 1.4 Critères de réception, déclarés avant construction

1. **Décomposition mesurée avant la construction** : coût par image contre le nombre de segments,
   dans les deux fenêtres ; la part imputable à la boucle interne en découle par la pente.
2. **Au bit** : `--multi --verify` rend **0,368476 mm** à 12 s avec 4 impacts ; `--multi --retour`
   rend **0 image différente** sur 31 comparées ; et le banc compare les **4 096 coefficients
   publiés** avant et après, au bit, sur une suite d'images.
3. **Coût publié** : banc avant et après ; puis `--multi --cadence` de l'hôte, CPU sillage médian,
   p95 et maximum, avec les allocations de `update` **toujours à zéro** (ADR-145).
4. **La revendication porte sur la croissance**, pas sur un point : la pente contre le nombre de
   segments doit tomber, et le dire.
5. **Coût** (ADR-131) : techniques présentes, absentes, domaine.
6. Aucun seuil modifié, aucune tolérance touchée, aucune ambition rouverte.

### 1.5 Arrêt

La sélection hissée, reçue au bit et chiffrée. **Ou** constat mesuré que la pente contre le nombre de
segments est négligeable — auquel cas la boucle interne n'est pas le poste, la mesure désigne le vrai,
et la session l'écrit sans rien construire.
