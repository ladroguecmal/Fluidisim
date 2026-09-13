# S210 — Inventaire des sources de l’hôte GPU

2026-09-13. Accord utilisateur « Oui » sur la résolution et les métadonnées, après S209.
**Sources non téléchargées ; accord sur leur récupération encore attendu.**

## Résultat de résolution

- Manifeste séparé : [viewer/Cargo.toml](../../viewer/Cargo.toml). Versions exactes
  `wgpu 30.0.1`, `winit 0.30.13`, `pollster 1.0.1`, fonctionnalités par défaut.
- [Verrou](../../viewer/Cargo.lock) : **256 paquets**, dont **254 du registre** et deux
  locaux (`water-core`, `water-viewer`). Le « Locking 255 packages » de Cargo compte
  les dépendances, dont le cœur local : le message du commit P2 « 255 externes » est erroné.
- Total des **254 archives** : **49,174,790 octets**, soit **49.17479 Mo**
  (décimal) ou **46.89673 Mio**. Espace extrait et compilation non mesurés.
- Toutes les sources sont `registry+https://github.com/rust-lang/crates.io-index`,
  identifiant canonique du registre crates.io ; le protocole utilisé est son index sparse.
  Aucune dépendance Git ni autre registre.
- 254 métadonnées de version reçues : aucun échec, licence et taille renseignées,
  aucun paquet retiré (`yanked=false`), 254 checksums API identiques au verrou.
- SHA-256 du fichier Cargo.lock : `6ae16d468decaad9b34ea308690879fba1d6483b9443a00f8521e13d0f088c8d`.

## Périmètre de la demande de sources

**La liste soumise est l’arbre portable complet ci-dessous**, pas un sous-ensemble Windows.
Il inclut des bibliothèques pour d’autres systèmes ; le sous-ensemble exact compilé sur
`x86_64-pc-windows-msvc` n’est pas établi ici. L’accord proposé couvre ces 254 versions
pour permettre la récupération du verrou entier sans nouvelle substitution de dépendance.
Les tailles sont celles des archives annoncées par le registre, non les octets effectivement
transférés (un cache local peut réduire le transfert).

Après accord sur ces sources : `cargo fetch --locked --manifest-path viewer/Cargo.toml`.
Puis compilation verrouillée et construction du véritable hôte. Le `main.rs` actuel est
**une cible vide de résolution**, jamais exécutée ni reçue comme application interactive.
Le cœur et son harnais conservent leur workspace sans dépendance externe.

**Conservation à choisir :** verrou et cache Cargo local, ou sources également copiées
dans le dépôt par vendoring. Ni vendoring ni archives versionnées dans cette session.

## Contrôles et limites

- `cargo generate-lockfile --locked` avec index sparse : succès, verrou inchangé.
- `cargo generate-lockfile --offline --locked` : **refus de modification du verrou**.
  Aucun contournement ni déverrouillage appliqué ; le cache présent ne reçoit pas cette
  régénération hors réseau. La raison précise n’a pas été diagnostiquée.
- Métadonnées de `code/` reçues avec `--offline --no-deps` : deux membres, cœur sans
  dépendance, harnais dépendant du seul cœur local. Aucun fichier de `code/` modifié.
- Rust local : 1.97.0, cible `x86_64-pc-windows-msvc`. Compatibilité de compilation
  non reçue sans les sources ; aucun build, test GPU ou téléchargement d’archives lancé.
- Réception numérique précédente : S208, 348 réussis/cinq ignorés, non rejouée ici.

## Inventaire exact

Source de chaque ligne : métadonnées publiques `https://crates.io/api/v1/crates/NOM/VERSION`,
lues le 2026-09-13 (lien sur le nom). Licences reproduites telles que déclarées par le
registre ; ce relevé ne remplace pas la lecture des notices livrées avec les sources.
Checksum SHA-256 de l’archive : comparaison API/verrou effectuée pour chaque ligne.

| Bibliothèque | Version | Licence déclarée | Archive (octets) | SHA-256 |
|---|---|---|---:|---|
| [ab_glyph](https://crates.io/api/v1/crates/ab_glyph/0.2.32) | 0.2.32 | Apache-2.0 | 20967 | `01c0457472c38ea5bd1c3b5ada5e368271cb550be7a4ca4a0b4634e9913f6cc2` |
| [ab_glyph_rasterizer](https://crates.io/api/v1/crates/ab_glyph_rasterizer/0.1.10) | 0.1.10 | Apache-2.0 | 11206 | `366ffbaa4442f4684d91e2cd7c5ea7c4ed8add41959a31447066e279e432b618` |
| [ahash](https://crates.io/api/v1/crates/ahash/0.8.12) | 0.8.12 | MIT OR Apache-2.0 | 43413 | `5a15f179cd60c4584b8a8c596927aadc462e27f2ca70c04e0071964a73ba7a75` |
| [allocator-api2](https://crates.io/api/v1/crates/allocator-api2/0.2.21) | 0.2.21 | MIT OR Apache-2.0 | 63622 | `683d7910e743518b0e34f1186f92494becacb047c7b6bf616c96772180fef923` |
| [android-activity](https://crates.io/api/v1/crates/android-activity/0.6.1) | 0.6.1 | MIT OR Apache-2.0 | 322611 | `0f2a1bb052857d5dd49572219344a7332b31b76405648eabac5bc68978251bcd` |
| [android-properties](https://crates.io/api/v1/crates/android-properties/0.2.2) | 0.2.2 | MIT | 4563 | `fc7eb209b1518d6bb87b283c20095f5228ecda460da70b44f0802523dea6da04` |
| [android_system_properties](https://crates.io/api/v1/crates/android_system_properties/0.1.6) | 0.1.6 | MIT OR Apache-2.0 | 5625 | `ae221649c9976a6f6c56ae1facf410f3ddb33cc661c4b7b61020a912d4237fbc` |
| [arrayref](https://crates.io/api/v1/crates/arrayref/0.3.9) | 0.3.9 | BSD-2-Clause | 9186 | `76a2e8124351fda1ef8aaaa3bbd7ebbcb486bbcd4225aca0aa0d84bb2db8fecb` |
| [arrayvec](https://crates.io/api/v1/crates/arrayvec/0.7.8) | 0.7.8 | MIT OR Apache-2.0 | 33260 | `d3fb67a6e08acf24fdeccbac2cb6ac4305825bd1f117462e0e6f2f193345ad56` |
| [as-raw-xcb-connection](https://crates.io/api/v1/crates/as-raw-xcb-connection/1.0.1) | 1.0.1 | MIT OR Apache-2.0 | 6460 | `175571dd1d178ced59193a6fc02dde1b972eb0bc56c892cde9beeceac5bf0f6b` |
| [ash](https://crates.io/api/v1/crates/ash/0.38.0+1.3.281) | 0.38.0+1.3.281 | MIT OR Apache-2.0 | 457775 | `0bb44936d800fea8f016d7f2311c6a4f97aebd5dc86f09906139ec848cf3a46f` |
| [atomic-waker](https://crates.io/api/v1/crates/atomic-waker/1.1.2) | 1.1.2 | Apache-2.0 OR MIT | 12422 | `1505bd5d3d116872e7271a6d4e16d81d0c8570876c8de68093a09ac269d8aac0` |
| [autocfg](https://crates.io/api/v1/crates/autocfg/1.5.1) | 1.5.1 | Apache-2.0 OR MIT | 18911 | `f2032f911046de80f0a198e0901378627c33f59ea0ac00e363d481118bd70a53` |
| [bit-set](https://crates.io/api/v1/crates/bit-set/0.10.0) | 0.10.0 | Apache-2.0 OR MIT | 20439 | `09ec2f926cc3060f09db9ebc5b52823d85268d24bb917e472c0c4bea35780a7d` |
| [bit-vec](https://crates.io/api/v1/crates/bit-vec/0.9.1) | 0.9.1 | Apache-2.0 OR MIT | 29177 | `b71798fca2c1fe1086445a7258a4bc81e6e49dcd24c8d0dd9a1e57395b603f51` |
| [bitflags](https://crates.io/api/v1/crates/bitflags/1.3.2) | 1.3.2 | MIT/Apache-2.0 | 23021 | `bef38d45163c2f1dde094a7dfd33ccf595c92905c8f8f4fdc18d06fb1037718a` |
| [bitflags](https://crates.io/api/v1/crates/bitflags/2.13.2) | 2.13.2 | MIT OR Apache-2.0 | 51678 | `3ded4057c258ba199e2d26386d3af3780957ecaee6c4ef4041c6b4b8b97c0b06` |
| [block2](https://crates.io/api/v1/crates/block2/0.5.1) | 0.5.1 | MIT | 24191 | `2c132eebf10f5cad5289222520a4a058514204aed6d791f1cf4fe8088b82d15f` |
| [block2](https://crates.io/api/v1/crates/block2/0.6.2) | 0.6.2 | MIT | 34505 | `cdeb9d870516001442e364c5220d3574d2da8dc765554b4a617230d33fa58ef5` |
| [bumpalo](https://crates.io/api/v1/crates/bumpalo/3.20.3) | 3.20.3 | MIT OR Apache-2.0 | 98671 | `72f5acc6cb2ba439de613abc23857ec3d78374d8ed5ac84e9d11336e87da8649` |
| [bytemuck](https://crates.io/api/v1/crates/bytemuck/1.25.2) | 1.25.2 | Zlib OR Apache-2.0 OR MIT | 54075 | `95832e849adfb21180ccb6826a99da14e5d266ae5c2e668e1602cf234f153797` |
| [bytemuck_derive](https://crates.io/api/v1/crates/bytemuck_derive/1.12.1) | 1.12.1 | Zlib OR Apache-2.0 OR MIT | 26509 | `6a1f896587b6f2c069c73d2f0913e2d590c3990285cd2f0b6aa02b786b4c679c` |
| [bytes](https://crates.io/api/v1/crates/bytes/1.12.1) | 1.12.1 | MIT | 75667 | `fc652a48c352aef3ea3aed32080501cf3ef6ed5da78602a020c991775b0aff04` |
| [calloop](https://crates.io/api/v1/crates/calloop/0.13.0) | 0.13.0 | MIT | 69698 | `b99da2f8558ca23c71f4fd15dc57c906239752dd27ff3c00a1d56b685b7cbfec` |
| [calloop-wayland-source](https://crates.io/api/v1/crates/calloop-wayland-source/0.3.0) | 0.3.0 | MIT | 11300 | `95a66a987056935f7efce4ab5668920b5d0dac4a7c99991a67395f13702ddd20` |
| [cc](https://crates.io/api/v1/crates/cc/1.4.5) | 1.4.5 | MIT OR Apache-2.0 | 101464 | `005ec2760ca554fae18df7a11195552ec576cd665632a881bc011d5bb2fd4d80` |
| [cfg-if](https://crates.io/api/v1/crates/cfg-if/1.0.4) | 1.0.4 | MIT OR Apache-2.0 | 9360 | `9330f8b2ff13f34540b44e946ef35111825727b38d33286ef986142615121801` |
| [cfg_aliases](https://crates.io/api/v1/crates/cfg_aliases/0.2.2) | 0.2.2 | MIT | 6527 | `f079e83a288787bcd14a6aea84cee5c87a67c5a3e660c30f557a3d24761b3527` |
| [codespan-reporting](https://crates.io/api/v1/crates/codespan-reporting/0.13.1) | 0.13.1 | Apache-2.0 | 57424 | `af491d569909a7e4dee0ad7db7f5341fef5c614d5b8ec8cf765732aba3cff681` |
| [combine](https://crates.io/api/v1/crates/combine/4.6.8) | 4.6.8 | MIT | 109123 | `cfc320937d09e6de266b31b9afb480f197d7a861be86be7cb2ea7e5d1bfffc5e` |
| [concurrent-queue](https://crates.io/api/v1/crates/concurrent-queue/2.5.0) | 2.5.0 | Apache-2.0 OR MIT | 22654 | `4ca0197aee26d1ae37445ee532fefce43251d24cc7c166799f4d46817f1d3973` |
| [core-foundation](https://crates.io/api/v1/crates/core-foundation/0.9.4) | 0.9.4 | MIT OR Apache-2.0 | 27743 | `91e195e091a93c46f7102ec7818a2aa394e1e1771c3ab4825963fa03e45afb8f` |
| [core-foundation-sys](https://crates.io/api/v1/crates/core-foundation-sys/0.8.7) | 0.8.7 | MIT OR Apache-2.0 | 37712 | `773648b94d0e5d620f64f280777445740e61fe701025087ec8b57f45c791888b` |
| [core-graphics](https://crates.io/api/v1/crates/core-graphics/0.23.2) | 0.23.2 | MIT OR Apache-2.0 | 30763 | `c07782be35f9e1140080c6b96f0d44b739e2278479f64e02fdab4e32dfd8b081` |
| [core-graphics-types](https://crates.io/api/v1/crates/core-graphics-types/0.1.3) | 0.1.3 | MIT OR Apache-2.0 | 7063 | `45390e6114f68f718cc7a830514a96f903cccd70d02a8f6d9f643ac4ba45afaf` |
| [crossbeam-utils](https://crates.io/api/v1/crates/crossbeam-utils/0.8.23) | 0.8.23 | MIT OR Apache-2.0 | 47174 | `a31eee39dddec8330830986fcd7625edb5a24ec90ea038215273bbc3adb08ac6` |
| [crunchy](https://crates.io/api/v1/crates/crunchy/0.2.4) | 0.2.4 | MIT | 3887 | `460fbee9c2c2f33933d720630a6a0bac33ba7053db5344fac858d4b8952d77d5` |
| [cursor-icon](https://crates.io/api/v1/crates/cursor-icon/1.2.0) | 1.2.0 | MIT OR Apache-2.0 OR Zlib | 14267 | `f27ae1dd37df86211c42e150270f82743308803d90a6f6e6651cd730d5e1732f` |
| [dispatch](https://crates.io/api/v1/crates/dispatch/0.2.0) | 0.2.0 | MIT | 10229 | `bd0c93bb4b0c6d9b77f4435b0ae98c24d17f1c45b2ff844c6151a07256ca923b` |
| [dispatch2](https://crates.io/api/v1/crates/dispatch2/0.3.1) | 0.3.1 | Zlib OR Apache-2.0 OR MIT | 55299 | `1e0e367e4e7da84520dedcac1901e4da967309406d1e51017ae1abfb97adbd38` |
| [dlib](https://crates.io/api/v1/crates/dlib/0.5.3) | 0.5.3 | MIT | 7120 | `ab8ecd87370524b461f8557c119c405552c396ed91fc0a8eec68679eab26f94a` |
| [document-features](https://crates.io/api/v1/crates/document-features/0.2.12) | 0.2.12 | MIT OR Apache-2.0 | 14739 | `d4b8a88685455ed29a21542a33abd9cb6510b6b129abadabdcef0f4c55bc8f61` |
| [downcast-rs](https://crates.io/api/v1/crates/downcast-rs/1.2.1) | 1.2.1 | MIT/Apache-2.0 | 11821 | `75b325c5dbd37f80359721ad39aca5a29fb04c89279657cffdda8736d0c0b9d2` |
| [dpi](https://crates.io/api/v1/crates/dpi/0.1.2) | 0.1.2 | Apache-2.0 AND MIT | 14812 | `d8b14ccef22fc6f5a8f4d7d768562a182c04ce9a3b3157b91390b52ddfdf1a76` |
| [equivalent](https://crates.io/api/v1/crates/equivalent/1.0.2) | 1.0.2 | Apache-2.0 OR MIT | 7419 | `877a4ace8713b0bcf2a4e7eec82529c029f1d0619886d18145fea96c3ffe5c0f` |
| [errno](https://crates.io/api/v1/crates/errno/0.3.14) | 0.3.14 | MIT OR Apache-2.0 | 12002 | `39cab71617ae0d63f51a36d69f866391735b51691dbda63cf6f96d042b63efeb` |
| [find-msvc-tools](https://crates.io/api/v1/crates/find-msvc-tools/0.1.12) | 0.1.12 | MIT OR Apache-2.0 | 31570 | `3e0f1c7c3a72c66fd80abe965175f7523475c0489a87d3ff9d6e8c87d87a9d2d` |
| [foldhash](https://crates.io/api/v1/crates/foldhash/0.2.0) | 0.2.0 | Zlib | 23329 | `77ce24cb58228fbb8aa041425bb1050850ac19177686ea6e0f41a70416f56fdb` |
| [foreign-types](https://crates.io/api/v1/crates/foreign-types/0.5.0) | 0.5.0 | MIT/Apache-2.0 | 7824 | `d737d9aa519fb7b749cbc3b962edcf310a8dd1f4b67c91c4f83975dbdd17d965` |
| [foreign-types-macros](https://crates.io/api/v1/crates/foreign-types-macros/0.2.4) | 0.2.4 | MIT/Apache-2.0 | 8161 | `ea5190182e6915eb873ddbc16e23b711b6eb1f9c00a0d0a3a91b5f6228475225` |
| [foreign-types-shared](https://crates.io/api/v1/crates/foreign-types-shared/0.3.1) | 0.3.1 | MIT/Apache-2.0 | 6006 | `aa9a19cbb55df58761df49b23516a86d432839add4af60fc256da840f66ed35b` |
| [futures-core](https://crates.io/api/v1/crates/futures-core/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | 14692 | `92d699e522242e69e3003b94ecc1f960f3a5e015aa7c5d7486e65ad01dd94f5e` |
| [futures-task](https://crates.io/api/v1/crates/futures-task/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | 11376 | `cd417de3d1d015fc3bfd2b1ea46dfc7bab72ef86f1cc7cc9c78e728b34a6d1fd` |
| [futures-util](https://crates.io/api/v1/crates/futures-util/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | 169782 | `0d50a92467f8ba5dd6e3ee5d4bd04d73ab2e4e1c44474a0674821dfce14b79bc` |
| [gethostname](https://crates.io/api/v1/crates/gethostname/1.1.0) | 1.1.0 | Apache-2.0 | 9865 | `1bd49230192a3797a9a4d6abe9b3eed6f7fa4c8a8a4947977c6f80025f92cbd8` |
| [getrandom](https://crates.io/api/v1/crates/getrandom/0.3.4) | 0.3.4 | MIT OR Apache-2.0 | 50932 | `899def5c37c4fd7b2664648c28120ecec138e4d395b459e5ca34f9cce2dd77fd` |
| [getrandom](https://crates.io/api/v1/crates/getrandom/0.4.3) | 0.4.3 | MIT OR Apache-2.0 | 52437 | `300e883d756b2e4ec94e02791f39b04b522276138852cfc41d9fb7e904106099` |
| [gl_generator](https://crates.io/api/v1/crates/gl_generator/0.14.0) | 0.14.0 | Apache-2.0 | 22330 | `1a95dfc23a2b4a9a2f5ab41d194f8bfda3cabec42af4e39f08c339eb2a0c124d` |
| [glow](https://crates.io/api/v1/crates/glow/0.17.0) | 0.17.0 | MIT OR Apache-2.0 OR Zlib | 165357 | `29038e1c483364cc6bb3cf78feee1816002e127c331a1eec55a4d202b9e1adb5` |
| [glutin_wgl_sys](https://crates.io/api/v1/crates/glutin_wgl_sys/0.6.1) | 0.6.1 | Apache-2.0 | 5567 | `2c4ee00b289aba7a9e5306d57c2d05499b2e5dc427f84ac708bd2c090212cf3e` |
| [gpu-allocator](https://crates.io/api/v1/crates/gpu-allocator/0.28.0) | 0.28.0 | MIT OR Apache-2.0 | 54791 | `51255ea7cfaadb6c5f1528d43e92a82acb2b96c43365989a28b2d44ee38f8795` |
| [half](https://crates.io/api/v1/crates/half/2.7.1) | 2.7.1 | MIT OR Apache-2.0 | 61040 | `6ea2d84b969582b4b1864a92dc5d27cd2b77b622a8d79306834f1be5ba20d84b` |
| [hashbrown](https://crates.io/api/v1/crates/hashbrown/0.16.1) | 0.16.1 | MIT OR Apache-2.0 | 147785 | `841d1cc9bed7f9236f321df977030373f4a4163ae1a7dbfe1a51a2c1a51d9100` |
| [hashbrown](https://crates.io/api/v1/crates/hashbrown/0.17.1) | 0.17.1 | MIT OR Apache-2.0 | 155512 | `ed5909b6e89a2db4456e54cd5f673791d7eca6732202bbf2a9cc504fe2f9b84a` |
| [hermit-abi](https://crates.io/api/v1/crates/hermit-abi/0.5.3) | 0.5.3 | MIT OR Apache-2.0 | 17458 | `e17592d60ebacc7d5e169f4663c5f84f9161cc90328abcfe8456f41e4dfcb284` |
| [indexmap](https://crates.io/api/v1/crates/indexmap/2.14.2) | 2.14.2 | Apache-2.0 OR MIT | 103014 | `cc4e190f5d26ca7051642629da2c52fc03bde85a03197c99408dcd291734c855` |
| [jni](https://crates.io/api/v1/crates/jni/0.22.4) | 0.22.4 | MIT OR Apache-2.0 | 199347 | `5efd9a482cf3a427f00d6b35f14332adc7902ce91efb778580e180ff90fa3498` |
| [jni-macros](https://crates.io/api/v1/crates/jni-macros/0.22.4) | 0.22.4 | MIT OR Apache-2.0 | 63557 | `a00109accc170f0bdb141fed3e393c565b6f5e072365c3bd58f5b062591560a3` |
| [jni-sys](https://crates.io/api/v1/crates/jni-sys/0.3.1) | 0.3.1 | MIT OR Apache-2.0 | 11570 | `41a652e1f9b6e0275df1f15b32661cf0d4b78d4d87ddec5e0c3c20f097433258` |
| [jni-sys](https://crates.io/api/v1/crates/jni-sys/0.4.1) | 0.4.1 | MIT OR Apache-2.0 | 13249 | `c6377a88cb3910bee9b0fa88d4f42e1d2da8e79915598f65fb0c7ee14c878af2` |
| [jni-sys-macros](https://crates.io/api/v1/crates/jni-sys-macros/0.4.1) | 0.4.1 | MIT OR Apache-2.0 | 3301 | `38c0b942f458fe50cdac086d2f946512305e5631e720728f2a61aabcd47a6264` |
| [jobserver](https://crates.io/api/v1/crates/jobserver/0.1.35) | 0.1.35 | MIT OR Apache-2.0 | 28890 | `1c00acbd29eabad4a2392fa0e921c874934dbbf4194312ad20f04a0ed67a3cb3` |
| [js-sys](https://crates.io/api/v1/crates/js-sys/0.3.105) | 0.3.105 | MIT OR Apache-2.0 | 113002 | `ce57d20d1ea864ce2ac172ab472d409214f4fd359f0b2a2775abdf522e2af99e` |
| [khronos-egl](https://crates.io/api/v1/crates/khronos-egl/6.0.0) | 6.0.0 | MIT/Apache-2.0 | 29315 | `6aae1df220ece3c0ada96b8153459b67eebe9ae9212258bb0134ae60416fdf76` |
| [khronos_api](https://crates.io/api/v1/crates/khronos_api/3.1.0) | 3.1.0 | Apache-2.0 | 599718 | `e2db585e1d738fc771bf08a151420d3ed193d9d895a36df7f6f8a9456b911ddc` |
| [libc](https://crates.io/api/v1/crates/libc/0.2.189) | 0.2.189 | MIT OR Apache-2.0 | 851502 | `3eaf3ede3fee6db1a4c2ee091bf8a8b4dccdc6d17f656fb07896ee72867612f2` |
| [libloading](https://crates.io/api/v1/crates/libloading/0.8.9) | 0.8.9 | ISC | 30222 | `d7c4b02199fee7c5d21a5ae7d8cfa79a6ef5bb2fc834d6e9058e89c825efdc55` |
| [libm](https://crates.io/api/v1/crates/libm/0.2.16) | 0.2.16 | MIT | 164243 | `b6d2cec3eae94f9f509c767b45932f1ada8350c4bdb85af2fcab4a3c14807981` |
| [libredox](https://crates.io/api/v1/crates/libredox/0.1.24) | 0.1.24 | MIT | 10567 | `6480ccc157a1389bb2e4891b24751b0f798ba640d22386f23143fbcc89da195a` |
| [linux-raw-sys](https://crates.io/api/v1/crates/linux-raw-sys/0.12.1) | 0.12.1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | 3006116 | `32a66949e030da00e8c7d4434b251670a91556f4144941d37452769c25d58a53` |
| [linux-raw-sys](https://crates.io/api/v1/crates/linux-raw-sys/0.4.15) | 0.4.15 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | 2150898 | `d26c52dbd32dccf2d10cac7725f8eae5296885fb5703b261f7d0a0739ec807ab` |
| [litrs](https://crates.io/api/v1/crates/litrs/1.0.0) | 1.0.0 | MIT OR Apache-2.0 | 46610 | `11d3d7f243d5c5a8b9bb5d6dd2b1602c0cb0b9db1621bafc7ed66e35ff9fe092` |
| [lock_api](https://crates.io/api/v1/crates/lock_api/0.4.14) | 0.4.14 | MIT OR Apache-2.0 | 29249 | `224399e74b87b5f3557511d98dff8b14089b3dadafcab6bb93eab67d3aace965` |
| [log](https://crates.io/api/v1/crates/log/0.4.34) | 0.4.34 | MIT OR Apache-2.0 | 53395 | `f9f8bd3e56ce4dfc153cf470fffbfa98c7620958b312ca5c3a4b8d5181fd13c6` |
| [memchr](https://crates.io/api/v1/crates/memchr/2.8.3) | 2.8.3 | Unlicense OR MIT | 99165 | `cf8baf1c55e62ffcace7a9f06f4bd9cd3f0c4beb022d3b367256b91b87513d98` |
| [memmap2](https://crates.io/api/v1/crates/memmap2/0.9.11) | 0.9.11 | MIT OR Apache-2.0 | 35116 | `d1219ed1b7f229ee7104d281dd01d6802fe28bb6e95d292942c4daacdeb798c0` |
| [naga](https://crates.io/api/v1/crates/naga/30.0.1) | 30.0.1 | MIT OR Apache-2.0 | 871520 | `a616d2fb8c89516ac2723a581f69d6c18576046bed761bd6b305e5618e6ae130` |
| [naga-types](https://crates.io/api/v1/crates/naga-types/30.0.1) | 30.0.1 | MIT OR Apache-2.0 | 12127 | `590afbf58a6f4f62873cd5cff4468061844bafa1cdf399cc954537c22d768d49` |
| [ndk](https://crates.io/api/v1/crates/ndk/0.9.0) | 0.9.0 | MIT OR Apache-2.0 | 84865 | `c3f42e7bbe13d351b6bead8286a43aac9534b82bd3cc43e47037f012ebfd62d4` |
| [ndk-context](https://crates.io/api/v1/crates/ndk-context/0.1.1) | 0.1.1 | MIT OR Apache-2.0 | 2205 | `27b02d87554356db9e9a873add8782d4ea6e3e58ea071a9adb9a2e8ddb884a8b` |
| [ndk-sys](https://crates.io/api/v1/crates/ndk-sys/0.6.0+11769913) | 0.6.0+11769913 | MIT OR Apache-2.0 | 329984 | `ee6cda3051665f1fb8d9e08fc35c96d5a244fb1be711a03b71118828afc9a873` |
| [num-traits](https://crates.io/api/v1/crates/num-traits/0.2.19) | 0.2.19 | MIT OR Apache-2.0 | 51631 | `071dfc062690e90b734c0b2273ce72ad0ffa95f0c74596bc250dcfd960262841` |
| [num_enum](https://crates.io/api/v1/crates/num_enum/0.7.6) | 0.7.6 | BSD-3-Clause OR MIT OR Apache-2.0 | 21741 | `5d0bca838442ec211fa11de3a8b0e0e8f3a4522575b5c4c06ed722e005036f26` |
| [num_enum_derive](https://crates.io/api/v1/crates/num_enum_derive/0.7.6) | 0.7.6 | BSD-3-Clause OR MIT OR Apache-2.0 | 18789 | `680998035259dcfcafe653688bf2aa6d3e2dc05e98be6ab46afb089dc84f1df8` |
| [objc-sys](https://crates.io/api/v1/crates/objc-sys/0.3.5) | 0.3.5 | MIT | 20560 | `cdb91bdd390c7ce1a8607f35f3ca7151b65afc0ff5ff3b34fa350f7d7c7e4310` |
| [objc2](https://crates.io/api/v1/crates/objc2/0.5.2) | 0.5.2 | MIT | 199204 | `46a785d4eeff09c14c487497c162e92766fbb3e4059a71840cecc03d9a50b804` |
| [objc2](https://crates.io/api/v1/crates/objc2/0.6.4) | 0.6.4 | MIT | 275200 | `3a12a8ed07aefc768292f076dc3ac8c48f3781c8f2d5851dd3d98950e8c5a89f` |
| [objc2-app-kit](https://crates.io/api/v1/crates/objc2-app-kit/0.2.2) | 0.2.2 | MIT | 337435 | `e4e89ad9e3d7d297152b17d39ed92cd50ca8063a89a9fa569046d41568891eff` |
| [objc2-cloud-kit](https://crates.io/api/v1/crates/objc2-cloud-kit/0.2.2) | 0.2.2 | MIT | 27094 | `74dd3b56391c7a0596a295029734d3c1c5e7e510a4cb30245f8221ccea96b009` |
| [objc2-contacts](https://crates.io/api/v1/crates/objc2-contacts/0.2.2) | 0.2.2 | MIT | 17094 | `a5ff520e9c33812fd374d8deecef01d4a840e7b41862d849513de77e44aa4889` |
| [objc2-core-data](https://crates.io/api/v1/crates/objc2-core-data/0.2.2) | 0.2.2 | MIT | 30575 | `617fbf49e071c178c0b24c080767db52958f716d9eabdf0890523aeae54773ef` |
| [objc2-core-foundation](https://crates.io/api/v1/crates/objc2-core-foundation/0.3.2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT | 180804 | `2a180dd8642fa45cdb7dd721cd4c11b1cadd4929ce112ebd8b9f5803cc79d536` |
| [objc2-core-graphics](https://crates.io/api/v1/crates/objc2-core-graphics/0.3.2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT | 84981 | `e022c9d066895efa1345f8e33e584b9f958da2fd4cd116792e15e07e4720a807` |
| [objc2-core-image](https://crates.io/api/v1/crates/objc2-core-image/0.2.2) | 0.2.2 | MIT | 20050 | `55260963a527c99f1819c4f8e3b47fe04f9650694ef348ffd2227e8196d34c80` |
| [objc2-core-location](https://crates.io/api/v1/crates/objc2-core-location/0.2.2) | 0.2.2 | MIT | 13100 | `000cfee34e683244f284252ee206a27953279d370e309649dc3ee317b37e5781` |
| [objc2-encode](https://crates.io/api/v1/crates/objc2-encode/4.1.0) | 4.1.0 | MIT | 21004 | `ef25abbcd74fb2609453eb695bd2f860d389e457f67dc17cafc8b8cbc89d0c33` |
| [objc2-foundation](https://crates.io/api/v1/crates/objc2-foundation/0.2.2) | 0.2.2 | MIT | 249330 | `0ee638a5da3799329310ad4cfa62fbf045d5f56e3ef5ba4149e7452dcf89d5a8` |
| [objc2-foundation](https://crates.io/api/v1/crates/objc2-foundation/0.3.2) | 0.3.2 | MIT | 345384 | `e3e0adef53c21f888deb4fa59fc59f7eb17404926ee8a6f59f5df0fd7f9f3272` |
| [objc2-io-surface](https://crates.io/api/v1/crates/objc2-io-surface/0.3.2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT | 11722 | `180788110936d59bab6bd83b6060ffdfffb3b922ba1396b312ae795e1de9d81d` |
| [objc2-link-presentation](https://crates.io/api/v1/crates/objc2-link-presentation/0.2.2) | 0.2.2 | MIT | 4440 | `a1a1ae721c5e35be65f01a03b6d2ac13a54cb4fa70d8a5da293d7b0020261398` |
| [objc2-metal](https://crates.io/api/v1/crates/objc2-metal/0.2.2) | 0.2.2 | MIT | 75276 | `dd0cba1276f6023976a406a14ffa85e1fdd19df6b0f737b063b95f6c8c7aadd6` |
| [objc2-metal](https://crates.io/api/v1/crates/objc2-metal/0.3.2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT | 226999 | `a0125f776a10d00af4152d74616409f0d4a2053a6f57fa5b7d6aa2854ac04794` |
| [objc2-quartz-core](https://crates.io/api/v1/crates/objc2-quartz-core/0.2.2) | 0.2.2 | MIT | 21306 | `e42bee7bff906b14b167da2bac5efe6b6a07e6f7c0a21a7308d40c960242dc7a` |
| [objc2-quartz-core](https://crates.io/api/v1/crates/objc2-quartz-core/0.3.2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT | 30036 | `96c1358452b371bf9f104e21ec536d37a650eb10f7ee379fff67d2e08d537f1f` |
| [objc2-symbols](https://crates.io/api/v1/crates/objc2-symbols/0.2.2) | 0.2.2 | MIT | 3626 | `0a684efe3dec1b305badae1a28f6555f6ddd3bb2c2267896782858d5a78404dc` |
| [objc2-ui-kit](https://crates.io/api/v1/crates/objc2-ui-kit/0.2.2) | 0.2.2 | MIT | 303111 | `b8bb46798b20cd6b91cbd113524c490f1686f4c4e8f49502431415f3512e2b6f` |
| [objc2-uniform-type-identifiers](https://crates.io/api/v1/crates/objc2-uniform-type-identifiers/0.2.2) | 0.2.2 | MIT | 6017 | `44fa5f9748dbfe1ca6c0b79ad20725a11eca7c2218bceb4b005cb1be26273bfe` |
| [objc2-user-notifications](https://crates.io/api/v1/crates/objc2-user-notifications/0.2.2) | 0.2.2 | MIT | 9600 | `76cfcbf642358e8689af64cee815d139339f3ed8ad05103ed5eaf73db8d84cb3` |
| [once_cell](https://crates.io/api/v1/crates/once_cell/1.21.4) | 1.21.4 | MIT OR Apache-2.0 | 35010 | `9f7c3e4beb33f85d45ae3e3a1792185706c8e16d043238c593331cc7cd313b50` |
| [orbclient](https://crates.io/api/v1/crates/orbclient/0.3.55) | 0.3.55 | MIT | 1341189 | `5df339f526ea9a60e371768d50efc2f2508c7203290731565d1f7a6f71d21747` |
| [ordered-float](https://crates.io/api/v1/crates/ordered-float/5.5.0) | 5.5.0 | MIT | 34706 | `8c7c9e0d9b23589f26070720bac724174bfec1083e82f7854cdd0267518343c0` |
| [owned_ttf_parser](https://crates.io/api/v1/crates/owned_ttf_parser/0.25.1) | 0.25.1 | Apache-2.0 | 8989 | `36820e9051aca1014ddc75770aab4d68bc1e9e632f0f5627c4086bc216fb583b` |
| [parking_lot](https://crates.io/api/v1/crates/parking_lot/0.12.5) | 0.12.5 | MIT OR Apache-2.0 | 46735 | `93857453250e3077bd71ff98b6a65ea6621a19bb0f559a85248955ac12c45a1a` |
| [parking_lot_core](https://crates.io/api/v1/crates/parking_lot_core/0.9.12) | 0.9.12 | MIT OR Apache-2.0 | 34110 | `2621685985a2ebf1c516881c026032ac7deafcda1a2c9b7850dc81e3dfcb64c1` |
| [percent-encoding](https://crates.io/api/v1/crates/percent-encoding/2.3.2) | 2.3.2 | MIT OR Apache-2.0 | 11583 | `9b4f627cb1b25917193a259e49bdad08f671f8d9708acfd5fe0a8c1455d87220` |
| [pin-project](https://crates.io/api/v1/crates/pin-project/1.1.13) | 1.1.13 | Apache-2.0 OR MIT | 56345 | `2466b2336ed02bcdca6b294417127b90ec92038d1d5c4fbeac971a922e0e0924` |
| [pin-project-internal](https://crates.io/api/v1/crates/pin-project-internal/1.1.13) | 1.1.13 | Apache-2.0 OR MIT | 29331 | `c96395f0a926bc13b1c17622aaddda1ecb55d49c8f1bf9777e4d877800a43f8b` |
| [pin-project-lite](https://crates.io/api/v1/crates/pin-project-lite/0.2.17) | 0.2.17 | Apache-2.0 OR MIT | 31034 | `a89322df9ebe1c1578d689c92318e070967d1042b512afbe49518723f4e6d5cd` |
| [pkg-config](https://crates.io/api/v1/crates/pkg-config/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | 21370 | `f6b464fbc74e149a392436b17d523f769e057cb6877f6a5c4618bc6f11800548` |
| [plain](https://crates.io/api/v1/crates/plain/0.2.3) | 0.2.3 | MIT/Apache-2.0 | 10664 | `b4596b6d070b27117e987119b4dac604f3c58cfb0b191112e24771b2faeac1a6` |
| [polling](https://crates.io/api/v1/crates/polling/3.11.0) | 3.11.0 | Apache-2.0 OR MIT | 59387 | `5d0e4f59085d47d8241c88ead0f274e8a0cb551f3625263c05eb8dd897c34218` |
| [pollster](https://crates.io/api/v1/crates/pollster/1.0.1) | 1.0.1 | Apache-2.0/MIT | 10102 | `bc6355899e1c9462875b6757c79f3caa011a1fdae12bbb1a2e72dd1f234f8336` |
| [portable-atomic](https://crates.io/api/v1/crates/portable-atomic/1.15.0) | 1.15.0 | Apache-2.0 OR MIT | 202705 | `05c8b63e8d9609db387f0324918f81d68fe27748f084ef092fb35954d0539a85` |
| [portable-atomic-util](https://crates.io/api/v1/crates/portable-atomic-util/0.2.8) | 0.2.8 | Apache-2.0 OR MIT | 53874 | `10ab3eb7f3becc3a1cbc4f2c6f20267996cfc1a6467a873763411b136a122715` |
| [presser](https://crates.io/api/v1/crates/presser/0.3.1) | 0.3.1 | MIT OR Apache-2.0 | 20946 | `e8cf8e6a8aa66ce33f63993ffc4ea4271eb5b0530a9002db8455ea6050c77bfa` |
| [proc-macro-crate](https://crates.io/api/v1/crates/proc-macro-crate/3.5.0) | 3.5.0 | MIT OR Apache-2.0 | 12709 | `e67ba7e9b2b56446f1d419b1d807906278ffa1a658a8a5d8a39dcb1f5a78614f` |
| [proc-macro2](https://crates.io/api/v1/crates/proc-macro2/1.0.107) | 1.0.107 | MIT OR Apache-2.0 | 59588 | `985e7ec9bb745e6ce6535b544d84d6cd6f7ad8bd711c398938ae983b91a766d9` |
| [profiling](https://crates.io/api/v1/crates/profiling/1.0.18) | 1.0.18 | MIT OR Apache-2.0 | 11931 | `3d595e54a326bc53c1c197b32d295e14b169e3cfeaa8dc82b529f947fba6bcf5` |
| [quick-xml](https://crates.io/api/v1/crates/quick-xml/0.41.0) | 0.41.0 | MIT | 231661 | `e660451e55124f798a69a5af3f49ccfbefbd41910eefd25caf2393e1f3473ec1` |
| [quote](https://crates.io/api/v1/crates/quote/1.0.47) | 1.0.47 | MIT OR Apache-2.0 | 31622 | `1fbf4db142a473a8d80c26bbf18454ed458bf8d26c8219c331daecfdbd079001` |
| [r-efi](https://crates.io/api/v1/crates/r-efi/5.3.0) | 5.3.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | 64532 | `69cdb34c158ceb288df11e18b4bd39de994f6657d83847bdffdbd7f346754b0f` |
| [r-efi](https://crates.io/api/v1/crates/r-efi/6.0.0) | 6.0.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | 65303 | `f8dcc9c7d52a811697d2151c701e0d08956f92b0e24136cf4cf27b57a6a0d9bf` |
| [range-alloc](https://crates.io/api/v1/crates/range-alloc/0.1.5) | 0.1.5 | MIT OR Apache-2.0 | 11163 | `ca45419789ae5a7899559e9512e58ca889e41f04f1f2445e9f4b290ceccd1d08` |
| [raw-window-handle](https://crates.io/api/v1/crates/raw-window-handle/0.6.2) | 0.6.2 | MIT OR Apache-2.0 OR Zlib | 20234 | `20675572f6f24e9e76ef639bc5552774ed45f1c30e2951e1e99c59888861c539` |
| [raw-window-metal](https://crates.io/api/v1/crates/raw-window-metal/1.1.0) | 1.1.0 | MIT OR Apache-2.0 | 15277 | `40d213455a5f1dc59214213c7330e074ddf8114c9a42411eb890c767357ce135` |
| [redox_syscall](https://crates.io/api/v1/crates/redox_syscall/0.4.1) | 0.4.1 | MIT | 24858 | `4722d768eff46b75989dd134e5c353f0d6296e5aaa3132e776cbdb56be7731aa` |
| [redox_syscall](https://crates.io/api/v1/crates/redox_syscall/0.5.18) | 0.5.18 | MIT | 30747 | `ed2bf2547551a7053d6fdfafda3f938979645c44812fbfcda098faae3f1a362d` |
| [redox_syscall](https://crates.io/api/v1/crates/redox_syscall/0.9.4) | 0.9.4 | MIT | 27944 | `737970939a87c6fa31e7acad13307bccbb017a073b695b6089a2c484f929e20e` |
| [renderdoc-sys](https://crates.io/api/v1/crates/renderdoc-sys/1.1.0) | 1.1.0 | MIT OR Apache-2.0 | 10366 | `19b30a45b0cd0bcca8037f3d0dc3421eaf95327a17cad11964fb8179b4fc4832` |
| [rustc-hash](https://crates.io/api/v1/crates/rustc-hash/1.1.0) | 1.1.0 | Apache-2.0/MIT | 9331 | `08d43f7aa6b08d49f382cde6a7982047c3426db949b1424bc4b7ec9ae12c6ce2` |
| [rustc_version](https://crates.io/api/v1/crates/rustc_version/0.4.1) | 0.4.1 | MIT OR Apache-2.0 | 12245 | `cfcb3a22ef46e85b45de6ee7e79d063319ebb6594faafcf1c225ea92ab6e9b92` |
| [rustix](https://crates.io/api/v1/crates/rustix/0.38.44) | 0.38.44 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | 379347 | `fdb5bc1ae2baa591800df16c9ca78619bf65c0488b41b96ccec5d11220d8c154` |
| [rustix](https://crates.io/api/v1/crates/rustix/1.1.4) | 1.1.4 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | 425241 | `b6fe4565b9518b83ef4f91bb47ce29620ca828bd32cb7e408f0062e9930ba190` |
| [rustversion](https://crates.io/api/v1/crates/rustversion/1.0.23) | 1.0.23 | MIT OR Apache-2.0 | 21013 | `cf54715a573b99ac80df0bc206da022bcd442c974952c7b9720069370852e21f` |
| [same-file](https://crates.io/api/v1/crates/same-file/1.0.6) | 1.0.6 | Unlicense/MIT | 10183 | `93fc1dc3aaa9bfed95e02e6eadabb4baf7e3078b0bd1b4d7b6b0b68378900502` |
| [scoped-tls](https://crates.io/api/v1/crates/scoped-tls/1.0.1) | 1.0.1 | MIT/Apache-2.0 | 8202 | `e1cf6437eb19a8f4a6cc0f7dca544973b0b78843adbfeb3683d1a94a0024a294` |
| [scopeguard](https://crates.io/api/v1/crates/scopeguard/1.2.0) | 1.2.0 | MIT OR Apache-2.0 | 11619 | `94143f37725109f92c262ed2cf5e59bce7498c01bcc1502d7b9afe439a4e9f49` |
| [sctk-adwaita](https://crates.io/api/v1/crates/sctk-adwaita/0.10.1) | 0.10.1 | MIT | 53237 | `b6277f0217056f77f1d8f49f2950ac6c278c0d607c45f5ee99328d792ede24ec` |
| [semver](https://crates.io/api/v1/crates/semver/1.0.28) | 1.0.28 | MIT OR Apache-2.0 | 33064 | `8a7852d02fc848982e0c167ef163aaff9cd91dc640ba85e263cb1ce46fae51cd` |
| [serde](https://crates.io/api/v1/crates/serde/1.0.229) | 1.0.229 | MIT OR Apache-2.0 | 83669 | `4148590afebada386688f18773da617792bf2ef03ffc1e4cbd2b1d45b023e0ba` |
| [serde_core](https://crates.io/api/v1/crates/serde_core/1.0.229) | 1.0.229 | MIT OR Apache-2.0 | 63100 | `67dca2c9c51e58a4791a4b1ed58308b39c64224d349a935ab5039aa360942a48` |
| [serde_derive](https://crates.io/api/v1/crates/serde_derive/1.0.229) | 1.0.229 | MIT OR Apache-2.0 | 59864 | `e7a5d71263a5a7d47b41f6b3f06ba276f10cc18b0931f1799f710578e2309348` |
| [shlex](https://crates.io/api/v1/crates/shlex/2.0.1) | 2.0.1 | MIT OR Apache-2.0 | 19332 | `f8fadd59c855ef2080decdef8ff161eb6661b86933c9d82e5ba29dc602a55aba` |
| [simd_cesu8](https://crates.io/api/v1/crates/simd_cesu8/1.2.0) | 1.2.0 | Apache-2.0 OR MIT | 320024 | `11031e251abf8611c80f460e19dbdeb54a66db918e49c65a7065b46ac7aec520` |
| [simdutf8](https://crates.io/api/v1/crates/simdutf8/0.1.5) | 0.1.5 | MIT OR Apache-2.0 | 28488 | `e3a9fe34e3e7a50316060351f37187a3f546bce95496156754b601a5fa71b76e` |
| [slab](https://crates.io/api/v1/crates/slab/0.4.12) | 0.4.12 | MIT | 19080 | `0c790de23124f9ab44544d7ac05d60440adc586479ce501c1d6d7da3cd8c9cf5` |
| [slotmap](https://crates.io/api/v1/crates/slotmap/1.1.1) | 1.1.1 | Zlib | 61862 | `bdd58c3c93c3d278ca835519292445cb4b0d4dc59ccfdf7ceadaab3f8aeb4038` |
| [smallvec](https://crates.io/api/v1/crates/smallvec/1.16.1) | 1.16.1 | MIT OR Apache-2.0 | 32694 | `ba467056f1b547ed52077911161fc86985becbc60e8e1857c8a144dab0def891` |
| [smithay-client-toolkit](https://crates.io/api/v1/crates/smithay-client-toolkit/0.19.2) | 0.19.2 | MIT | 131504 | `3457dea1f0eb631b4034d61d4d8c32074caa6cd1ab2d59f2327bd8461e2c0016` |
| [smol_str](https://crates.io/api/v1/crates/smol_str/0.2.2) | 0.2.2 | MIT OR Apache-2.0 | 15840 | `dd538fb6910ac1099850255cf94a94df6551fbdd602454387d0adb2d1ca6dead` |
| [spirv](https://crates.io/api/v1/crates/spirv/0.4.0+sdk-1.4.341.0) | 0.4.0+sdk-1.4.341.0 | Apache-2.0 | 41285 | `d9571ea910ebd84c86af4b3ed27f9dbdc6ad06f17c5f96146b2b671e2976744f` |
| [static_assertions](https://crates.io/api/v1/crates/static_assertions/1.1.0) | 1.1.0 | MIT OR Apache-2.0 | 18480 | `a2eb9349b6444b326872e140eb1cf5e7c522154d69e7a0ffb0fb81c06b37543f` |
| [strict-num](https://crates.io/api/v1/crates/strict-num/0.1.1) | 0.1.1 | MIT | 5104 | `6637bab7722d379c8b41ba849228d680cc12d0a45ba1fa2b48f2a30577a06731` |
| [syn](https://crates.io/api/v1/crates/syn/2.0.119) | 2.0.119 | MIT OR Apache-2.0 | 307407 | `872831b642d1a07999a962a351ed35b955ea2cfc8f3862091e2a240a84f17297` |
| [syn](https://crates.io/api/v1/crates/syn/3.0.5) | 3.0.5 | MIT OR Apache-2.0 | 313222 | `12df2e0110f65b775f769bb17ef989067a1d931b2eb822bd4346631eeada89f9` |
| [termcolor](https://crates.io/api/v1/crates/termcolor/1.4.1) | 1.4.1 | Unlicense OR MIT | 18773 | `06794f8f6c5c898b3275aebefa6b8a1cb24cd2c6c79397ab15774837a0bc5755` |
| [thiserror](https://crates.io/api/v1/crates/thiserror/1.0.69) | 1.0.69 | MIT OR Apache-2.0 | 22198 | `b6aaf5339b578ea85b50e080feb250a3e8ae8cfcdff9a461c9ec2904bc923f52` |
| [thiserror](https://crates.io/api/v1/crates/thiserror/2.0.20) | 2.0.20 | MIT OR Apache-2.0 | 28969 | `ec86235f5fcc2a73650310756d2ac5b138a5780bbbdfae3eeccec992c435ba4f` |
| [thiserror-impl](https://crates.io/api/v1/crates/thiserror-impl/1.0.69) | 1.0.69 | MIT OR Apache-2.0 | 18365 | `4fee6c4efc90059e10f81e6d42c60a18f76588c3d74cb83a0b242a2b6c7504c1` |
| [thiserror-impl](https://crates.io/api/v1/crates/thiserror-impl/2.0.20) | 2.0.20 | MIT OR Apache-2.0 | 21436 | `bc04cd3e1236dd4a98afca4569f2deb3f120e5422a4023be2cb683f8486292af` |
| [tiny-skia](https://crates.io/api/v1/crates/tiny-skia/0.11.4) | 0.11.4 | BSD-3-Clause | 201082 | `83d13394d44dae3207b52a326c0c85a8bf87f1541f23b0d143811088497b09ab` |
| [tiny-skia-path](https://crates.io/api/v1/crates/tiny-skia-path/0.11.4) | 0.11.4 | BSD-3-Clause | 47764 | `9c9e7fc0c2e86a30b117d0462aa261b72b7a99b7ebd7deb3a14ceda95c5bdc93` |
| [toml_datetime](https://crates.io/api/v1/crates/toml_datetime/1.1.1+spec-1.1.0) | 1.1.1+spec-1.1.0 | MIT OR Apache-2.0 | 17982 | `3165f65f62e28e0115a00b2ebdd37eb6f3b641855f9d636d3cd4103767159ad7` |
| [toml_edit](https://crates.io/api/v1/crates/toml_edit/0.25.15+spec-1.1.0) | 0.25.15+spec-1.1.0 | MIT OR Apache-2.0 | 68207 | `1340ea94a5856333492c9064b02c778b191dd2c853778d9609debdcdfea3a614` |
| [toml_parser](https://crates.io/api/v1/crates/toml_parser/1.1.3+spec-1.1.0) | 1.1.3+spec-1.1.0 | MIT OR Apache-2.0 | 35888 | `1d38ac1cf9b95face32296c0a3ede1fdc270627c9d9c02a7274dd6d960dc4d56` |
| [tracing](https://crates.io/api/v1/crates/tracing/0.1.44) | 0.1.44 | MIT | 463135 | `63e71662fa4b2a2c3a26f570f037eb95bb1f85397f3cd8076caed2f026a6d100` |
| [tracing-core](https://crates.io/api/v1/crates/tracing-core/0.1.36) | 0.1.36 | MIT | 63967 | `db97caf9d906fbde555dd62fa95ddba9eecfd14cb388e4f491a66d74cd5fb79a` |
| [ttf-parser](https://crates.io/api/v1/crates/ttf-parser/0.25.1) | 0.25.1 | MIT OR Apache-2.0 | 201121 | `d2df906b07856748fa3f6e0ad0cbaa047052d4a7dd609e231c4f72cee8c36f31` |
| [unicode-ident](https://crates.io/api/v1/crates/unicode-ident/1.0.24) | 1.0.24 | (MIT OR Apache-2.0) AND Unicode-3.0 | 49298 | `e6e4313cd5fcd3dad5cafa179702e2b244f760991f45397d14d4ebf38247da75` |
| [unicode-segmentation](https://crates.io/api/v1/crates/unicode-segmentation/1.13.3) | 1.13.3 | MIT OR Apache-2.0 | 112325 | `c6f5d3c3b1bf09027a88a6bc961fc00497d651009560b5463668dc81b0fa87a8` |
| [unicode-width](https://crates.io/api/v1/crates/unicode-width/0.2.2) | 0.2.2 | MIT OR Apache-2.0 | 282768 | `b4ac048d71ede7ee76d585517add45da530660ef4390e49b098733c6e897f254` |
| [version_check](https://crates.io/api/v1/crates/version_check/0.9.5) | 0.9.5 | MIT/Apache-2.0 | 15554 | `0b928f33d975fc6ad9f86c8f283853ad26bdd5b10b7f1542aa2fa15e2289105a` |
| [walkdir](https://crates.io/api/v1/crates/walkdir/2.5.0) | 2.5.0 | Unlicense/MIT | 23951 | `29790946404f91d9c5d06f9874efddea1dc06c5efe94541a7d6863108e3a5e4b` |
| [wasip2](https://crates.io/api/v1/crates/wasip2/1.0.4+wasi-0.2.12) | 1.0.4+wasi-0.2.12 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | 135311 | `b67efb37e106e55ce722a510d6b5f9c17f083e5fc79afc2badeb12cc313d9487` |
| [wasm-bindgen](https://crates.io/api/v1/crates/wasm-bindgen/0.2.128) | 0.2.128 | MIT OR Apache-2.0 | 70345 | `aecb87a33d3b0c5e3b7aa46336eaf486cffafbd281b195e4c8b80d50df2351bf` |
| [wasm-bindgen-futures](https://crates.io/api/v1/crates/wasm-bindgen-futures/0.4.78) | 0.4.78 | MIT OR Apache-2.0 | 8291 | `6ef4c5d3d2cdf5c54f4231181768f5510842e350db025faf1f7163b1030ed928` |
| [wasm-bindgen-macro](https://crates.io/api/v1/crates/wasm-bindgen-macro/0.2.128) | 0.2.128 | MIT OR Apache-2.0 | 9595 | `a690d511e3c1a8b3a55e33511e3c2c00c78415cd23650f32b808627f5696b9ed` |
| [wasm-bindgen-macro-support](https://crates.io/api/v1/crates/wasm-bindgen-macro-support/0.2.128) | 0.2.128 | MIT OR Apache-2.0 | 118512 | `411e4887f0071ef2d2164a9d5fdf2d20efbef78fccd3a78b0c10a1dc5295e48a` |
| [wasm-bindgen-shared](https://crates.io/api/v1/crates/wasm-bindgen-shared/0.2.128) | 0.2.128 | MIT OR Apache-2.0 | 12994 | `81941cd78d0c92026c33e5e01312845a4cb1e9af3407f9134b100dd03144103e` |
| [wayland-backend](https://crates.io/api/v1/crates/wayland-backend/0.3.17) | 0.3.17 | MIT | 81255 | `38a91b4eaddff87b1cd1074985e3713da4af2c49742d1b356b2c01670a67a078` |
| [wayland-client](https://crates.io/api/v1/crates/wayland-client/0.31.15) | 0.31.15 | MIT | 67091 | `e3c36a0f861ad76d0901f2800b46321410d9f73f2ea88aac0650d86c32688073` |
| [wayland-csd-frame](https://crates.io/api/v1/crates/wayland-csd-frame/0.3.0) | 0.3.0 | MIT | 5696 | `625c5029dbd43d25e6aa9615e88b829a5cad13b2819c4ae129fdbb7c31ab4c7e` |
| [wayland-cursor](https://crates.io/api/v1/crates/wayland-cursor/0.31.14) | 0.31.14 | MIT | 9331 | `4a52d18780be9b1314328a3de5f930b73d2200112e3849ca6cb11822793fb34d` |
| [wayland-protocols](https://crates.io/api/v1/crates/wayland-protocols/0.32.13) | 0.32.13 | MIT | 162090 | `23d0c813de3daa2ed6520af85a3bd49b0e722a3078506899aa9686fea58dc4b6` |
| [wayland-protocols-plasma](https://crates.io/api/v1/crates/wayland-protocols-plasma/0.3.12) | 0.3.12 | MIT | 68806 | `2b6d8cf1eb2c1c31ed1f5643c88a6e53538129d4af80030c8cabd1f9fa884d91` |
| [wayland-protocols-wlr](https://crates.io/api/v1/crates/wayland-protocols-wlr/0.3.12) | 0.3.12 | MIT | 28366 | `eb04e52f7836d7c7976c78ca0250d61e33873c34156a2a1fc9474828ec268234` |
| [wayland-scanner](https://crates.io/api/v1/crates/wayland-scanner/0.31.11) | 0.31.11 | MIT | 37536 | `338e30461b3a2b67d70eb30a6d89f8e0c93a833e07d2ae89085cd070c4a00ac0` |
| [wayland-sys](https://crates.io/api/v1/crates/wayland-sys/0.31.11) | 0.31.11 | MIT | 9727 | `d8eab23fefc9e41f8e841df4a9c707e8a8c4ed26e944ef69297184de2785e3be` |
| [web-sys](https://crates.io/api/v1/crates/web-sys/0.3.105) | 0.3.105 | MIT OR Apache-2.0 | 682289 | `9fbddc4a036f00ec4f18c83445bd3115cb306a91da554919a099d9222fe4a7f8` |
| [web-time](https://crates.io/api/v1/crates/web-time/1.1.0) | 1.1.0 | MIT OR Apache-2.0 | 18026 | `5a6580f308b1fad9207618087a65c04e7a10bc77e02c8e84e9b00dd4b12fa0bb` |
| [wgpu](https://crates.io/api/v1/crates/wgpu/30.0.1) | 30.0.1 | MIT OR Apache-2.0 | 231453 | `527ccdf43dd5b2e8676eed9984ce00e2bbb0a1b85b70c1969dcb6cd2eb55ab9e` |
| [wgpu-core](https://crates.io/api/v1/crates/wgpu-core/30.0.1) | 30.0.1 | MIT OR Apache-2.0 | 386299 | `14c018fce9b6270aa203c2fdd56f3cce996713534bd757e4ea58c8560b121f14` |
| [wgpu-core-deps-apple](https://crates.io/api/v1/crates/wgpu-core-deps-apple/30.0.1) | 30.0.1 | MIT OR Apache-2.0 | 12523 | `061f3d319a40d39d00b1ecc2c33b89fe21d4e6fe01859df3500a3a8ecccd6b68` |
| [wgpu-core-deps-emscripten](https://crates.io/api/v1/crates/wgpu-core-deps-emscripten/30.0.1) | 30.0.1 | MIT OR Apache-2.0 | 11296 | `d98b86cf4abf524a902dd35f18ca6a3f08fc2ae9847c8f10b48e30491b1f0b86` |
| [wgpu-core-deps-windows-linux-android](https://crates.io/api/v1/crates/wgpu-core-deps-windows-linux-android/30.0.1) | 30.0.1 | MIT OR Apache-2.0 | 13070 | `7586165fd5f6d881cb9ce4bb71f40d6caab2c0f1837e3fc1d9788a197fb6004f` |
| [wgpu-hal](https://crates.io/api/v1/crates/wgpu-hal/30.0.1) | 30.0.1 | MIT OR Apache-2.0 | 466434 | `b6b7fb58561a792bc237628ba0792e332de418fefe145f13b5ed8201e6d52f58` |
| [wgpu-naga-bridge](https://crates.io/api/v1/crates/wgpu-naga-bridge/30.0.1) | 30.0.1 | MIT OR Apache-2.0 | 9624 | `d2f62e73117bb7a62bfd9c5a5841438a823f6566c6442a808ee269d2d055c081` |
| [wgpu-types](https://crates.io/api/v1/crates/wgpu-types/30.0.1) | 30.0.1 | MIT OR Apache-2.0 | 144370 | `99dad6f1fbdbbdb4c278a6508b059d44688f5cebddf78d005a46a31340269286` |
| [winapi-util](https://crates.io/api/v1/crates/winapi-util/0.1.11) | 0.1.11 | Unlicense OR MIT | 13368 | `c2a7b1c03c876122aa43f3020e6c3c3ee5c05081c9a00739faf7503aeba10d22` |
| [windows](https://crates.io/api/v1/crates/windows/0.62.2) | 0.62.2 | MIT OR Apache-2.0 | 9360572 | `527fadee13e0c05939a6a05d5bd6eec6cd2e3dbd648b9f8e447c6518133d8580` |
| [windows-collections](https://crates.io/api/v1/crates/windows-collections/0.3.2) | 0.3.2 | MIT OR Apache-2.0 | 13510 | `23b2d95af1a8a14a3c7367e1ed4fc9c20e0a26e79551b1454d72583c97cc6610` |
| [windows-core](https://crates.io/api/v1/crates/windows-core/0.62.2) | 0.62.2 | MIT OR Apache-2.0 | 36932 | `b8e83a14d34d0623b51dce9581199302a221863196a1dde71a7663a4c2be9deb` |
| [windows-future](https://crates.io/api/v1/crates/windows-future/0.3.2) | 0.3.2 | MIT OR Apache-2.0 | 17944 | `e1d6f90251fe18a279739e78025bd6ddc52a7e22f921070ccdc67dde84c605cb` |
| [windows-implement](https://crates.io/api/v1/crates/windows-implement/0.60.2) | 0.60.2 | MIT OR Apache-2.0 | 15325 | `053e2e040ab57b9dc951b72c264860db7eb3b0200ba345b4e4c3b14f67855ddf` |
| [windows-interface](https://crates.io/api/v1/crates/windows-interface/0.59.3) | 0.59.3 | MIT OR Apache-2.0 | 11809 | `3f316c4a2570ba26bbec722032c4099d8c8bc095efccdc15688708623367e358` |
| [windows-link](https://crates.io/api/v1/crates/windows-link/0.2.1) | 0.2.1 | MIT OR Apache-2.0 | 6133 | `f0805222e57f7521d6a62e36fa9163bc891acd422f971defe97d64e70d0a4fe5` |
| [windows-numerics](https://crates.io/api/v1/crates/windows-numerics/0.3.1) | 0.3.1 | MIT OR Apache-2.0 | 9772 | `6e2e40844ac143cdb44aead537bbf727de9b044e107a0f1220392177d15b0f26` |
| [windows-result](https://crates.io/api/v1/crates/windows-result/0.4.1) | 0.4.1 | MIT OR Apache-2.0 | 13381 | `7781fa89eaf60850ac3d2da7af8e5242a5ea78d1a11c49bf2910bb5a73853eb5` |
| [windows-strings](https://crates.io/api/v1/crates/windows-strings/0.5.1) | 0.5.1 | MIT OR Apache-2.0 | 13966 | `7837d08f69c77cf6b07689544538e017c1bfcf57e34b4c0ff58e6c2cd3b37091` |
| [windows-sys](https://crates.io/api/v1/crates/windows-sys/0.52.0) | 0.52.0 | MIT OR Apache-2.0 | 2576877 | `282be5f36a8ce781fad8c8ae18fa3f9beff57ec1b52cb3de0789201425d9a33d` |
| [windows-sys](https://crates.io/api/v1/crates/windows-sys/0.59.0) | 0.59.0 | MIT OR Apache-2.0 | 2387323 | `1e38bc4d79ed67fd075bcc251a1c39b32a1776bbe92e5bef1f0bf1f8c531853b` |
| [windows-sys](https://crates.io/api/v1/crates/windows-sys/0.61.2) | 0.61.2 | MIT OR Apache-2.0 | 2517186 | `ae137229bcbd6cdf0f7b80a31df61766145077ddf49416a728b02cb3921ff3fc` |
| [windows-targets](https://crates.io/api/v1/crates/windows-targets/0.52.6) | 0.52.6 | MIT OR Apache-2.0 | 6403 | `9b724f72796e036ab90c1021d4780d4d3d648aca59e491e6b98e725b84e99973` |
| [windows-threading](https://crates.io/api/v1/crates/windows-threading/0.2.1) | 0.2.1 | MIT OR Apache-2.0 | 9686 | `3949bd5b99cafdf1c7ca86b43ca564028dfe27d66958f2470940f73d86d75b37` |
| [windows_aarch64_gnullvm](https://crates.io/api/v1/crates/windows_aarch64_gnullvm/0.52.6) | 0.52.6 | MIT OR Apache-2.0 | 435718 | `32a4622180e7a0ec044bb555404c800bc9fd9ec262ec147edd5989ccd0c02cd3` |
| [windows_aarch64_msvc](https://crates.io/api/v1/crates/windows_aarch64_msvc/0.52.6) | 0.52.6 | MIT OR Apache-2.0 | 832615 | `09ec2a7bb152e2252b53fa7803150007879548bc709c039df7627cabbd05d469` |
| [windows_i686_gnu](https://crates.io/api/v1/crates/windows_i686_gnu/0.52.6) | 0.52.6 | MIT OR Apache-2.0 | 880402 | `8e9b5ad5ab802e97eb8e295ac6720e509ee4c243f69d781394014ebfe8bbfa0b` |
| [windows_i686_gnullvm](https://crates.io/api/v1/crates/windows_i686_gnullvm/0.52.6) | 0.52.6 | MIT OR Apache-2.0 | 475940 | `0eee52d38c090b3caa76c563b86c3a4bd71ef1a819287c19d586d7334ae8ed66` |
| [windows_i686_msvc](https://crates.io/api/v1/crates/windows_i686_msvc/0.52.6) | 0.52.6 | MIT OR Apache-2.0 | 901163 | `240948bc05c5e7c6dabba28bf89d89ffce3e303022809e73deaefe4f6ec56c66` |
| [windows_x86_64_gnu](https://crates.io/api/v1/crates/windows_x86_64_gnu/0.52.6) | 0.52.6 | MIT OR Apache-2.0 | 836363 | `147a5c80aabfbf0c7d901cb5895d1de30ef2907eb21fbbab29ca94c5b08b1a78` |
| [windows_x86_64_gnullvm](https://crates.io/api/v1/crates/windows_x86_64_gnullvm/0.52.6) | 0.52.6 | MIT OR Apache-2.0 | 435707 | `24d5b23dc417412679681396f2b49f3de8c1473deb516bd34410872eff51ed0d` |
| [windows_x86_64_msvc](https://crates.io/api/v1/crates/windows_x86_64_msvc/0.52.6) | 0.52.6 | MIT OR Apache-2.0 | 832564 | `589f6da84c646204747d1270a2a5661ea66ed1cced2631d546fdfb155959f9ec` |
| [winit](https://crates.io/api/v1/crates/winit/0.30.13) | 0.30.13 | Apache-2.0 | 605180 | `a6755fa58a9f8350bd1e472d4c3fcc25f824ec358933bba33306d0b63df5978d` |
| [winnow](https://crates.io/api/v1/crates/winnow/1.0.4) | 1.0.4 | MIT | 188513 | `23b97319f7b8343df12cc98938e5c3eb436064524c8d2b4e30a1d3a36eecdf81` |
| [wit-bindgen](https://crates.io/api/v1/crates/wit-bindgen/0.57.1) | 0.57.1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | 71227 | `1ebf944e87a7c253233ad6766e082e3cd714b5d03812acc24c318f549614536e` |
| [x11-dl](https://crates.io/api/v1/crates/x11-dl/2.21.0) | 2.21.0 | MIT | 66823 | `38735924fedd5314a6e548792904ed8c6de6636285cb9fec04d5b1db85c1516f` |
| [x11rb](https://crates.io/api/v1/crates/x11rb/0.13.2) | 0.13.2 | MIT OR Apache-2.0 | 220549 | `9993aa5be5a26815fe2c3eacfc1fde061fc1a1f094bf1ad2a18bf9c495dd7414` |
| [x11rb-protocol](https://crates.io/api/v1/crates/x11rb-protocol/0.13.2) | 0.13.2 | MIT OR Apache-2.0 | 514069 | `ea6fc2961e4ef194dcbfe56bb845534d0dc8098940c7e5c012a258bfec6701bd` |
| [xcursor](https://crates.io/api/v1/crates/xcursor/0.3.11) | 0.3.11 | MIT | 8292 | `163b33ed8786455e2fa5d72f554057ce3f3182425434f756cd39c99839d88e23` |
| [xkbcommon-dl](https://crates.io/api/v1/crates/xkbcommon-dl/0.4.2) | 0.4.2 | MIT | 5879 | `d039de8032a9a8856a6be89cea3e5d12fdd82306ab7c94d74e6deab2460651c5` |
| [xkeysym](https://crates.io/api/v1/crates/xkeysym/0.2.1) | 0.2.1 | MIT OR Apache-2.0 OR Zlib | 103129 | `b9cc00251562a284751c9973bace760d86c0276c471b4be569fe6b068ee97a56` |
| [xml-rs](https://crates.io/api/v1/crates/xml-rs/0.8.29) | 0.8.29 | MIT | 52816 | `e450f9b2ed1dff33c94c12589a87338689467b9c4f5d8a5710bd09a847d2c8a7` |
| [zerocopy](https://crates.io/api/v1/crates/zerocopy/0.8.57) | 0.8.57 | BSD-2-Clause OR Apache-2.0 OR MIT | 285690 | `d35102a9f36d089ccae9e4c6802bc118be4487b80aaffc0ab4e0cf5ce92d2873` |
| [zerocopy-derive](https://crates.io/api/v1/crates/zerocopy-derive/0.8.57) | 0.8.57 | BSD-2-Clause OR Apache-2.0 OR MIT | 106175 | `146c01f5ab44258da43cf276c74a2763db2ff3969c9c652c3f2de07041d0b2bc` |

## Suite de construction

J1 reste actif : fenêtre/adaptateur, données B et phases repliées, impact par table λ/16,
comparaison CPU/GPU, coût GPU mesuré, puis sillage. A247/A250 restent ouvertes ; aucun
seuil ni périmètre changé. Voir [feuille de route](../FEUILLE-DE-ROUTE.md).
