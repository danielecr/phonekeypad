# Phone Keypad

Play the phone keypad: a sunday project.

## Prelude

I read a science informativ article on time and perception in psicological ordering.
It say we normally do not perceive the sequence of a phone keypad.
That does not convince me, so I wrote this code to ear it.

## rodio: the crate

https://crates.io/crates/rodio
I take crate rodio, basic examples are easy, I produce some sounds with SineWave
generator.

I was convinced phone keypad produce one SineWave for each key, so I can write it
easily.
Search for frequencies ...

> wikipedia: https://en.wikipedia.org/wiki/Telephone_keypad#Key_tones

A cleaner article:

> https://www.sigidwiki.com/wiki/Dual_Tone_Multi_Frequency_%28DTMF%29

So, no. I was wrong: a key produces 2 SineWave.

rodio has a mixer created by
`rodio::dynamic_mixer::mixer(channel, sample_rate)`

So 2 channel is exactly what I need.

SineWave produce sound at 48_000 sample rate, and this is ok.

But a sequence of keypad keys must be sequenced then splitted in two channel.

`source::from_iter()` accepts an iterator and create a Source.
So it take the ownership, I use
https://doc.rust-lang.org/nightly/std/iter/trait.Iterator.html#method.unzip

Ok this is the code.

## Future

(I mean, not "Rust Future"). I admit that this tool is very annoying, also
it is not possible to make it interactive.

Rodio provides 2 utilities for crafting a source:

- `from_iter()`
- `from_factory()`

`from_factory()` take a function. To make it interactive it must exists 2 function
that produce void SineWave (volume 0.0) until there is a input key.

A maybe future.