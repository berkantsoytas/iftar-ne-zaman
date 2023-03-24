# İftar Ne Zaman

Bulunduğunuz ülke, şehire bağlı iftara kalan vakti gösteren basit bir cli uygulaması.

<br/>

## Kullanım

Uygulamayı kullanabilmek için aşağıda ki talimatları okuyunuz.

### Rust Kurulumu

Rust'u indirmek için, [Rust resmi web sitesi](https://www.rust-lang.org/tools/install) adresindeki talimatları izleyin.

1. Öncelikle, Rust yüklü olduğundan emin olun. Bunu kontrol etmek için terminalde şu komutu çalıştırın:

   ```sh
   rustc --version
   ```

2. Öncelikle kaynak kodlarını indirin. Uygulamanın kodu main.rs adlı bir dosyada olmalıdır.

3. Terminal üzerinden proje klasörüne gidin.

   ```sh
   cd <proje_klasoru>
   ```

4. Projenin derlenmesi için aşağıda ki komutları sırayla çalıştırın:

   ```sh
   cargo build --release
   ```

   cargo build komutu, projeyi derler ve gerekli tüm bağımlılıkları indirir. --release bayrağı, derlenen uygulamanın daha hızlı ve optimize edilmiş bir sürümünü oluşturur.

   Eğer windows üzerinden linux'a derleyecekseniz lütfen bu flag'i ekleyiniz;
   `---target=x86_64-unknown-linux-gnu`

5. Derlenen uygulamayı çalıştırmak için aşağıdaki komutu kullanabilirsiniz:

   ```sh
   ./target/release/iftar_ne_zaman <Ulke> <Sehir> <Bolge>
   ```

   Bu komut, derlenen uygulamayı çalıştırır ve bulunduğunuz şehre göre iftar saatini gösterir.

6. Örnek bir kullanım aşağıda verilmiştir.

   ```sh
   ./target/release/iftar_ne_zaman Turkey Adana Adana
   ```

7. Çıktı ise aşağıda ki gibidir.

   ```sh
   Bugunun ibadet saatleri:
   +------------+-------+-------+-------+--------+-------+-------+
   | Tarih      | Imsak | Gunes | Ogle  | Ikindi | Aksam | Yatsi |
   +------------+-------+-------+-------+--------+-------+-------+
   | 2023-03-24 | 05:10 | 06:30 | 12:50 | 16:19  | 19:01 | 20:16 |
   +------------+-------+-------+-------+--------+-------+-------+
    Iftara Kalan Vakit:
    3 SAAT 23 DAKIKA

   ```
