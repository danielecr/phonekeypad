# Tastierino telefono

L'idea è molto semplice, anche se leggermente eterodossa, specialmente di domenica:

fare un tastierino del telefono.

## Antefatto

Leggo un'articolo divulgativo (percezione e tempo in psicologia) che parla della
difficoltà di riconoscere la sequenza di suoni prodotta da tastierino del telefono
(ad esemppio quando si usa "ripeti numero"), questa cosa non mi e decido di fare
qualche prova realistica, ma senza telefono.

## rodio: il crate

https://crates.io/crates/rodio
Decido di usare il crate rodio, gli esempi basilari sono facili da capire, e produco
qualche suono da un generatore sinusoidale.

Ora penso che il tastierino del telefono produca una sinusoide, quindi posso trovarne la frequenza.
Cerco ...

> wikipedia: https://en.wikipedia.org/wiki/Telephone_keypad#Key_tones

Effettivamente è poco chiaro

> https://www.sigidwiki.com/wiki/Dual_Tone_Multi_Frequency_%28DTMF%29

Ora è chiaro: ogni tasto del tastierino produce 2 note sinusoidali che vanno sommate.

La documentazione dice che esiste un mixer che può essere creato tramite
`rodio::dynamic_mixer::mixer(channel, sample_rate)`

Penso che 2 canali siano sufficienti, visto che devo produrre 2 sinusoidi contemporaneamente.

Ora, visto che il controller del mixer ha il metodo `add()` che accetta una sorgente
alla quale non si può più aggiungere nulla, tutta la sequenza dei suoni va preparata per ogni
canale.

Senza complicare le cose, un canale riproduce le "frequenze orizzontali", e l'altro le
"frequenze verticali".

Come faccio a sequenzializzare dei `SineWave`?

`source::from_iter()` accetta un iterator e crea una sorgente.
Tecnicamente _prende_ un iterator e crea una sorgente, cioè ne acquisisce l'ownership.

Infine trovo unzip(): https://doc.rust-lang.org/nightly/std/iter/trait.Iterator.html#method.unzip

Forse il progetto rimane così, intanto è un esempio di come usare mixer.