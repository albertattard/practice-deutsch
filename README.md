# Practice Deutsch

A simple command line application to practice German.  This application supports the following practices

1. Practice German articles (_der_, _die_, and _das_)

   ```shell
   $ practice-deutsch articles
   ```

   Listen to the noun and then type the article

2. Practice German numbers

   ```shell
   $ practice-deutsch numbers
   ```

   Listen to the number and then type the number

3. Practice German alphabet

   ```shell
   $ practice-deutsch alphabet
   ```

   Listen to the letter of the alphabet and then type it

## Resources

- [Nouns](https://www.verbformen.com/declension/nouns/)
- [Verb conjugation](https://conjugator.reverso.net/conjugation-german.html)
- [The German Project](https://www.thegermanproject.com/german-lessons)
- [Learn A1 Level German](https://langster.org/en/grammar/german/a1/)
- [Natural Readers](https://www.naturalreaders.com/online/)
- [Practice Materials](https://www.goethe.de/ins/de/en/prf/prf/gzsd1/ueb.html)
- [German with Laura](https://germanwithlaura.com/)
- [inlingua Audio Files](https://www.dropbox.com/sh/yxyw77z5woty2r2/AADzwD5IUjp_Du1WqVsoNe2Ja?dl=0)

## Useful Commands

1. Build the project

   ```shell
   $ cargo fmt
   $ cargo clippy
   $ cargo build --release
   $ cargo test
   ```

   This will create a binary file `./target/release/practice-deutsch`.

2. Run the project

   ```shell
   $ ./target/release/practice-deutsch --help
   ```
