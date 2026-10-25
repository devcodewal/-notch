# notch — v0.1 scaffold

Notch untuk Windows: pil kecil di bibir atas layar yang menampilkan musik,
lirik sinkron (satu baris berjalan saat pil tertutup, lirik lengkap saat pil
dibuka), timer fokus, todo, dan rak file. Dibangun dengan **Tauri v2**
(UI HTML/CSS/JS + backend Rust).

> Nama "notch" masih nama kerja. Ganti `productName` di
> `src-tauri/tauri.conf.json` dan `identifier`-nya kalau sudah ada nama final.

## Yang sudah jalan di scaffold ini

- Jendela notch: tanpa dekorasi, transparan, selalu di atas, tanpa ikon
  taskbar, nempel di tengah atas layar (340x36 tertutup, 560x470 terbuka).
- Musik dari pemutar apa pun lewat SMTC Windows (Spotify, Apple Music,
  YouTube di browser, dll): judul, artis, posisi, status play/pause,
  tombol play/pause, next, prev.
- Lirik sinkron dari LRCLIB (gratis, tanpa key): baris berjalan di pil
  tertutup, lirik lengkap dengan baris aktif saat pil dibuka.
- Pomodoro, todo, rak file (masih lokal di frontend, belum persisten).
- Mode demo: kalau `index.html` dibuka di browser biasa (bukan dari Tauri),
  tampil simulasi desktop + lagu demo, jadi desain bisa dicek tanpa build.

## Yang belum (rencana v0.2)

- Seek lagu dari klik baris lirik (saat ini klik baris hanya jalan di demo).
- Penyimpanan todo/rak/pengaturan (pakai plugin store Tauri).
- Auto-start Windows + ikon system tray + pengaturan.
- Cover album dari SMTC (saat ini ikon not balok generik).
- Hotkey global.

## Build di PC Windows

### 1. Install alat

1. **Rust**: jalankan `rustup-init.exe` dari https://rustup.rs, lalu buka
   terminal baru.
2. **Node.js LTS** dari https://nodejs.org.
3. **Visual Studio Build Tools 2022** dengan workload
   "Desktop development with C++" (untuk linker MSVC).
   Windows 11 sudah membawa WebView2. Windows 10 perlu install
   WebView2 Runtime dari Microsoft.

### 2. Jalankan

```powershell
cd notch-app
npm install
npx tauri dev
```

`tauri dev` membuka notch di tengah atas layar. Putar lagu di Spotify /
YouTube, lalu hover pilnya.

### 3. Build installer

```powershell
npx tauri build
```

Hasilnya di `src-tauri/target/release/bundle/nsis/`.

## Catatan jujur

Scaffold ini ditulis dan dicek strukturnya dari Linux (JSON valid,
frontend ter-render di browser mode demo). **Compile Rust + perilaku SMTC
belum terverifikasi** karena mesin penulis tidak punya toolchain Windows.
Build pertama di PC Windows kemungkinan memunculkan error kecil di crate
`windows` (misalnya nama API) — kirim pesan errornya, nanti diperbaiki.

## Struktur

```
notch-app/
├── package.json                 # CLI Tauri (devDependency)
├── src/
│   └── index.html               # seluruh UI + logika (dual-mode)
└── src-tauri/
    ├── Cargo.toml
    ├── build.rs
    ├── tauri.conf.json          # jendela 340x36, transparan, topmost
    ├── capabilities/default.json
    ├── icons/icon.png, icon.ico
    └── src/
        ├── main.rs
        ├── lib.rs               # command: get_media, media_toggle,
        │                        # media_next, media_prev, get_lyrics
        └── media.rs             # SMTC (Windows saja)
```

## Dapat .exe tanpa install toolchain (GitHub Actions)

Kalau tidak mau install Rust + Build Tools di PC, pakai jalur ini:

1. Buat repo GitHub baru, push seluruh isi folder `notch-app/` ke branch
   `main`.
2. Buka tab **Actions** di repo — workflow `publish` jalan otomatis.
3. Kalau selesai, buka **Releases** — ada draft release `notch v0.1.0`
   berisi installer `.exe` (NSIS). Download, jalankan, selesai.

Catatan: installer ini **belum signed** (belum ada sertifikat code
signing), jadi Windows SmartScreen akan menampilkan peringatan
"Windows protected your PC" — klik **More info** lalu **Run anyway**.
Ini normal untuk aplikasi indie baru, sama seperti yang dialami Nusa.
