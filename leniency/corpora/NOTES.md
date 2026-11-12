# Real-world corpus for ICNS, TGA, ICO/CUR

Purpose: measure whether the strict rules of our verified cores reject files that real producers write. All bytes, per-file manifests (source URL, repo, commit, licence, SHA-256), per-file results and anomaly inputs live under `/tmp/claude-0/corpora/<format>/`. Nothing from the corpora is stored in this repository; this file records only sources, counts and licences.

Acquisition: GitHub tree API plus raw.githubusercontent.com at a pinned commit for named decoder/test-suite repos; GitHub code search (`extension:`/`filename:` queries) for in-the-wild files, capped at 2 to 8 files per repo; shallow clones of two gitlab.gnome.org repos (gdk-pixbuf, glycin test-images). Files are deduplicated by SHA-256, capped at 5 MiB, and type-checked by magic/header plausibility rather than extension. Git LFS pointer files, symlink targets and HTML pages that carried these extensions were discarded from measurement (LFS content is served from media.githubusercontent.com, which is not reachable).

## ICNS

Downloaded and deduplicated: 835. Passed the type check and measured: 230 from 92 repos. Licence labels (GitHub-detected SPDX id of the repo, not verified per file): NOASSERTION/none-detected 103, MIT 81, NOASSERTION 15, Apache-2.0 9, GPL-3.0 6, BSD-3-Clause 6, CC-BY-4.0 6, LGPL-2.1+(see-repo) 1, Zlib 1, GPL-2.0 1, ISC 1.

| source repo | files | repo licence |
|---|---|---|
| mdsteele/rust-icns | 13 | MIT |
| blackosx/OpenCanopyIcons | 12 | NOASSERTION/none-detected |
| tinalatif/flat.icns | 12 | NOASSERTION/none-detected |
| godotengine/godot | 7 | MIT |
| haraldk/TwelveMonkeys | 6 | BSD-3-Clause |
| KElee01/macos-icons | 6 | NOASSERTION/none-detected |
| Legacy-Mac-Icons/legacy-mac-icons.github.io | 6 | NOASSERTION/none-detected |
| MiYogurt/glorify-mac-icon | 6 | NOASSERTION/none-detected |
| MisutaaAsriel/Azis-bigSurIcons | 6 | CC-BY-4.0 |
| amanchokshi/minimalist-monochrome-mac-icons | 6 | MIT |
| artemfromnl/macos-icon-changer | 6 | MIT |
| doekman/MS-Office_Mac_icons | 6 | NOASSERTION/none-detected |
| jeff-hykin/mac-icon-changer | 6 | NOASSERTION/none-detected |
| jesseweed/mac-icons | 6 | NOASSERTION/none-detected |
| lexrus/LTFinderButtons | 6 | MIT |
| mikemykhaylov/macIconPack | 6 | NOASSERTION/none-detected |
| mschoch/iOS-Couchbase-Xcode-templates | 6 | NOASSERTION/none-detected |
| yuuiko/mac-Icons | 6 | NOASSERTION/none-detected |
| qt/qtimageformats | 5 | NOASSERTION |
| qfish/Bee-Xcode-Template | 5 | MIT |
| python-pillow/Pillow | 4 | NOASSERTION |
| AloneMonkey/MonkeyDev-Xcode-Templates | 4 | Apache-2.0 |
| relikd/icnsutil | 3 | MIT |
| kokoabim/iOSOpenDev-Xcode-Templates | 3 | NOASSERTION |
| uwedeportivo/NikePlus-Cocoa-RunLog | 3 | MIT |
| image-rs/image-extras | 2 | Apache-2.0 |
| electron-userland/electron-builder | 2 | MIT |
| mohouyizme/vscode-big-sur-icons | 2 | MIT |
| LVS/AGP-Launcher | 2 | NOASSERTION/none-detected |
| MagicSketch/XcodeTemplate-SketchPlugin | 2 | MIT |
| drnic/choctop | 2 | NOASSERTION/none-detected |
| johnste/finicky | 2 | MIT |
| sindresorhus/create-dmg | 2 | MIT |
| ulises-jeremias/agent-toolkit | 1 | MIT |
| Qtrick/prebasecode | 1 | MIT |
| SECTL/Cicada | 1 | GPL-3.0 |
| gitlab.gnome.org/GNOME/gdk-pixbuf | 1 | LGPL-2.1+(see-repo) |
| libsdl-org/SDL | 1 | Zlib |
| kornelski/libicns | 1 | GPL-2.0 |
| electron/electron | 1 | MIT |
| Daij-Djan/DDMountainNotifier | 1 | MIT |
| alptugan/icns-creator | 1 | MIT |
| sprout2000/elephicon | 1 | MIT |
| starbudweiser/MiCONE | 1 | MIT |
| xiaozhuai/imageinfo | 1 | MIT |
| Dimillian/CodexSkillManager | 1 | MIT |
| Eef89/PNG2ICNS | 1 | MIT |
| Jingyuan-Zheng/macos-icon-finder | 1 | MIT |
| Juanpe/Swift-VIPER-Module | 1 | NOASSERTION/none-detected |
| MaksimKazachkov/iOS-Clean-Architecture-with-Coordinator-pattern | 1 | MIT |
| Maxmudjon/Get_MiHome_devices_token | 1 | NOASSERTION/none-detected |
| Nikita2k/SwiftViper | 1 | NOASSERTION/none-detected |
| ProfileCreator/ProfileCreator | 1 | MIT |
| ServiceStackApps/ReactChatApps | 1 | NOASSERTION/none-detected |
| Whikiey/tauri-icons-builder | 1 | Apache-2.0 |
| abdullahselek/ViperC | 1 | MIT |
| atduskgreg/drift | 1 | NOASSERTION/none-detected |
| barbuza/EasyMail | 1 | NOASSERTION/none-detected |
| bazz1tv/SDL2-Xcode-Template | 1 | NOASSERTION/none-detected |
| c9rgreen/emacs-macos-icon | 1 | GPL-3.0 |
| caged/cocos2d-game | 1 | NOASSERTION/none-detected |
| chris-swift-dev/TheComposableArchitecture-Xcode-Template | 1 | MIT |
| cloudapp/icon | 1 | NOASSERTION/none-detected |
| divijkumar/xcode-template-ane | 1 | NOASSERTION |
| dotvhs/steam-mac-icon | 1 | NOASSERTION/none-detected |
| dreamcat4/MediaKeys | 1 | NOASSERTION/none-detected |
| du-song/Xcode-Template-iOS-dylib | 1 | NOASSERTION/none-detected |
| eklipse2k8/CocoaRestClient | 1 | NOASSERTION |
| giftbott/XcodeTemplateHelper | 1 | MIT |
| haaakon/Scaffolding | 1 | NOASSERTION/none-detected |
| hughkli/Lookin | 1 | GPL-3.0 |
| iCasablanca/IconUtility | 1 | NOASSERTION/none-detected |
| infinum/ios-viper-xcode-templates | 1 | Apache-2.0 |
| itsjoshpark/charge-limiter | 1 | GPL-3.0 |
| jonathanchu/emacs-mac-icon | 1 | GPL-3.0 |
| kaishin/MacIconPreviewer | 1 | ISC |
| kineme/QCPatchXcodeTemplate | 1 | NOASSERTION/none-detected |
| kofigumbs/multi | 1 | GPL-3.0 |
| licatajustin/google-chrome-icon | 1 | NOASSERTION/none-detected |
| machinecodex/CodexFab_LicenseExample | 1 | NOASSERTION/none-detected |
| magicalpanda/Xcode-Templates | 1 | NOASSERTION/none-detected |
| mattt/Xcode-Licensed-Templates | 1 | MIT |
| mo-browser-apps/icons | 1 | MIT |
| moritree/tauri-linux-icon-mre | 1 | NOASSERTION/none-detected |
| null-none/png2icns | 1 | MIT |
| onmyway133/IconGenerator | 1 | NOASSERTION |
| ryekelly/NVRDump | 1 | NOASSERTION/none-detected |
| smallab/CCGLTouch-XCode-Templates | 1 | NOASSERTION/none-detected |
| sozercan/kaset | 1 | MIT |
| tldev/dorso | 1 | MIT |
| vinhnx/VTCode | 1 | Apache-2.0 |
| zqxwce/vphone-ws | 1 | MIT |

## TGA

Downloaded and deduplicated: 930. Passed the type check and measured: 300 from 48 repos. Licence labels (GitHub-detected SPDX id of the repo, not verified per file): NOASSERTION/none-detected 76, NOASSERTION 60, MIT 54, LGPL-2.1+(see-repo) 42, Apache-2.0 37, GPL-3.0 10, ISC 8, LGPL-2.1 6, Zlib 3, BSD-3-Clause 2, see-repo(mixed) 1, Unlicense 1.

| source repo | files | repo licence |
|---|---|---|
| gitlab.gnome.org/GNOME/gdk-pixbuf | 42 | LGPL-2.1+(see-repo) |
| KDE/kimageformats | 38 | NOASSERTION/none-detected |
| python-pillow/Pillow | 34 | NOASSERTION |
| image-rs/image | 21 | Apache-2.0 |
| AcademySoftwareFoundation/OpenImageIO | 13 | Apache-2.0 |
| libgd/libgd | 12 | NOASSERTION |
| assimp/assimp | 8 | NOASSERTION |
| bluescan/tacentview | 8 | ISC |
| cerviDOS/cursed-vga | 8 | GPL-3.0 |
| gitmike81/jack-cs2-sticker-kit | 8 | NOASSERTION/none-detected |
| jaloji/tga-viewer | 8 | MIT |
| lennartstolz/TGAImage | 8 | MIT |
| nameless323/Cookbook | 8 | NOASSERTION/none-detected |
| npedotnet/TGAReader | 8 | MIT |
| HappySeaFox/sail | 7 | MIT |
| OxideAV/oxideav-tga | 7 | MIT |
| DEF7/TGA2STL | 5 | NOASSERTION/none-detected |
| UnmovableLibrary/lab5 | 5 | NOASSERTION/none-detected |
| dimao8/libtgaloader | 5 | LGPL-2.1 |
| nickbabcock/Pfim | 4 | MIT |
| OpenImageIO/oiio-images | 3 | Apache-2.0 |
| google/wuffs | 3 | NOASSERTION |
| chux0519/tgaimage | 3 | NOASSERTION/none-detected |
| ftrvxmtrx/tga | 3 | MIT |
| Shervanator/SOIL | 2 | NOASSERTION/none-detected |
| haraldk/TwelveMonkeys | 2 | BSD-3-Clause |
| i9or/more-primitive-obj-loader | 2 | GPL-3.0 |
| josh-english-2k18/tga-loader | 2 | MIT |
| m1nuz/file2header | 2 | NOASSERTION/none-detected |
| r-lyeh-archived/spot | 2 | Zlib |
| raydelto/tga_loader | 2 | NOASSERTION/none-detected |
| ImageMagick/ImageMagick | 1 | NOASSERTION |
| qt/qtimageformats | 1 | NOASSERTION |
| libsdl-org/SDL_image | 1 | Zlib |
| gitlab.gnome.org/sophie-h/test-images | 1 | see-repo(mixed) |
| godotengine/godot | 1 | MIT |
| libvips/libvips | 1 | LGPL-2.1 |
| AM-young-fun/TgaLoader | 1 | NOASSERTION/none-detected |
| SuperIlu/DosView | 1 | NOASSERTION |
| WTFTrick/gl_tga_viewer | 1 | NOASSERTION/none-detected |
| Zi7ar21/C-Path-Tracer | 1 | MIT |
| aspose-imaging/Aspose.Imaging-for-Java | 1 | MIT |
| delphi-pascal-archive/targa | 1 | Unlicense |
| ksiazeblaznow/tga2png | 1 | MIT |
| ksm2/targa-service-worker | 1 | MIT |
| paulusmas/TGA | 1 | NOASSERTION/none-detected |
| vthibault/tga.js | 1 | MIT |
| xiaozhuai/imageinfo | 1 | MIT |

## ICO/CUR

Downloaded and deduplicated: 1268. Passed the type check and measured: 360 from 113 repos. Licence labels (GitHub-detected SPDX id of the repo, not verified per file): NOASSERTION/none-detected 141, MIT 51, GPL-3.0 49, NOASSERTION 48, Apache-2.0 22, LGPL-2.1 13, GPL-2.0 11, LGPL-2.1+(see-repo) 7, CC-BY-4.0 7, AGPL-3.0 3, see-repo(mixed) 2, Zlib 2, BSD-3-Clause 2, CC0-1.0 2.

| source repo | files | repo licence |
|---|---|---|
| image-rs/image | 16 | Apache-2.0 |
| python-pillow/Pillow | 12 | NOASSERTION |
| DentonW/DevIL | 10 | LGPL-2.1 |
| gitlab.gnome.org/GNOME/gdk-pixbuf | 7 | LGPL-2.1+(see-repo) |
| 0443n/dota2-cursors | 6 | NOASSERTION/none-detected |
| AaronsLifeGame/Mouse-Cursors | 6 | NOASSERTION/none-detected |
| Abablabab/Posys-cursor | 6 | NOASSERTION/none-detected |
| Curzyori/anime-cursor | 6 | GPL-3.0 |
| EmperorPenguin18/SkyrimCursor | 6 | GPL-3.0 |
| Jased-0001/ClickyCursorPack | 6 | NOASSERTION/none-detected |
| JonathanILevi/gfie-src | 6 | GPL-3.0 |
| Juwan-Hwang/macOS27-Cursors | 6 | NOASSERTION |
| RandumbGuy1/MeleeCursorPack | 6 | NOASSERTION/none-detected |
| ShumTin/Windows-11-Cursors-Concept-v2 | 6 | NOASSERTION/none-detected |
| Slackcraft/CraftCursors | 6 | NOASSERTION/none-detected |
| SullensCR/Windows-Material-Design-Cursor-V2-Dark-Hdpi-by-jepriCreations | 6 | NOASSERTION/none-detected |
| Suorkndnf/Wii-Cursor-Pack-by-Suorkndnf | 6 | NOASSERTION/none-detected |
| The-time-forever/Amiya_Cursor_Pack | 6 | CC-BY-4.0 |
| Theozou-159/KnightsCollegeCursor | 6 | NOASSERTION |
| aw3r1se/macOS-cursors | 6 | MIT |
| belizariogr/Hypnotic-Cursor-Pack | 6 | NOASSERTION/none-detected |
| c-cluster/cPink | 6 | GPL-3.0 |
| danielheth/nsis | 6 | NOASSERTION |
| eatguoking/tomoyo-cursor-packs | 6 | NOASSERTION/none-detected |
| emanuelghdev/silksong-cursor | 6 | NOASSERTION/none-detected |
| generalram/nicons | 6 | GPL-2.0 |
| keeferrourke/capitaine-cursors | 6 | NOASSERTION |
| licyk/ani2xcur | 6 | GPL-3.0 |
| luisrguerra/papirus-icon-theme-windows | 6 | GPL-3.0 |
| mashrakiwork/CustoMouse | 6 | NOASSERTION/none-detected |
| matecursorpack/matecursorpack | 6 | NOASSERTION/none-detected |
| mishzx/IconFlow | 6 | MIT |
| its-myusuf/MacOS-theme | 5 | NOASSERTION/none-detected |
| lab0102/Starcraft-Brood-War-cursor-pack-for-Windows | 5 | NOASSERTION/none-detected |
| liusuan233/cursor-manager | 5 | MIT |
| AcademySoftwareFoundation/OpenImageIO | 4 | Apache-2.0 |
| FranklinDM/Moonscape-IconPack | 4 | NOASSERTION |
| GordCaswell/gfieportable | 4 | NOASSERTION/none-detected |
| Pablitus666/Ico_Converter | 4 | NOASSERTION/none-detected |
| YAL-GameMaker/window_set_icon | 4 | NOASSERTION/none-detected |
| jmg/py2nsis | 4 | NOASSERTION/none-detected |
| komh/winico | 4 | GPL-2.0 |
| lunacys/IconEditor | 4 | NOASSERTION/none-detected |
| mat/besticon | 4 | MIT |
| mdsteele/rust-ico | 3 | MIT |
| libvips/libvips | 3 | LGPL-2.1 |
| StarsiegePlayers/installer | 3 | MIT |
| StroepWafel/LYTE-Installer | 3 | AGPL-3.0 |
| bgtwoigu/greenfish | 3 | GPL-3.0 |
| digidigital/nsi-designer | 3 | MIT |
| drmikecrowe/GTDInbox-Installer | 3 | NOASSERTION/none-detected |
| funwaywang/Bamboo | 3 | MIT |
| gitlab.gnome.org/sophie-h/test-images | 2 | see-repo(mixed) |
| ImageMagick/ImageMagick | 2 | NOASSERTION |
| libsdl-org/SDL_image | 2 | Zlib |
| pyglet/pyglet | 2 | BSD-3-Clause |
| godotengine/godot | 2 | MIT |
| HerbertLausmann/HL.IconPro | 2 | NOASSERTION/none-detected |
| b-com-software-basis/builddefs-qmake | 2 | NOASSERTION/none-detected |
| gpwen/vim-installer-mui2 | 2 | NOASSERTION/none-detected |
| insani-org/nsis-installer | 2 | CC0-1.0 |
| ivan-magky/electron-custom-windows-installer | 2 | NOASSERTION/none-detected |
| kefir500/apk-icon-editor | 2 | GPL-3.0 |
| OpenImageIO/oiio-images | 1 | Apache-2.0 |
| assimp/assimp | 1 | NOASSERTION |
| corkami/pocs | 1 | NOASSERTION/none-detected |
| google/wuffs | 1 | NOASSERTION |
| ArcherGu/ikon | 1 | MIT |
| Bahlaouane-Hamza/I-Miss-You | 1 | MIT |
| BenjaminRi/winresource | 1 | MIT |
| Carl00-mpeek/Roblox-Cursor-Studio | 1 | NOASSERTION |
| GehnShard/Installer | 1 | NOASSERTION/none-detected |
| Gemiyur/IconEditor | 1 | MIT |
| Gixx/img2svg-icon-converter | 1 | MIT |
| K-PRSC/Kael-Icon-Converter | 1 | GPL-3.0 |
| Logerfo/RBTray | 1 | GPL-2.0 |
| LukeProducts/Icon-Converter-Master | 1 | NOASSERTION/none-detected |
| MagicAl6244225/IVFixer | 1 | NOASSERTION/none-detected |
| Mr5niper/Pyicon-Editor | 1 | MIT |
| NC22/Volna42BW-Tools | 1 | GPL-3.0 |
| NixaVulpi/Image2Icon | 1 | MIT |
| ShreyasMohite/Icon-converter | 1 | MIT |
| SiL3NC3/SvgToPngIconConverter | 1 | MIT |
| Tang0314/IconCanvas | 1 | NOASSERTION/none-detected |
| TingtingG233/IconConverter | 1 | MIT |
| TorsionTools/Torsion.Apps.IconConverter | 1 | GPL-3.0 |
| Wren6991/ComputeModuleBootInstaller | 1 | NOASSERTION |
| X-Necro-X/Icon-Converter | 1 | NOASSERTION/none-detected |
| abhinavr-19/ico-converter | 1 | NOASSERTION/none-detected |
| aj-phillips/IconSwapperGui | 1 | GPL-3.0 |
| augustocesarperin/icosmith | 1 | MIT |
| dboycht/IconConverter | 1 | NOASSERTION/none-detected |
| duma520/Image_To_Icon_Converter | 1 | MIT |
| egorozh/Cross-Platform-Icon-Editor | 1 | NOASSERTION/none-detected |
| expressjs/serve-favicon | 1 | MIT |
| fallssyj/IcoConverter | 1 | MIT |
| filebot/filebot-installer | 1 | NOASSERTION/none-detected |
| guivieiras/IconConverter | 1 | NOASSERTION/none-detected |
| hf1985/husk-webcam | 1 | MIT |
| iShrxmp/8GIF | 1 | NOASSERTION |
| igorskyflyer/xcursor | 1 | GPL-3.0 |
| ilnanny75/ConverterPro | 1 | MIT |
| jamingray/Image-to-ICO-Converter | 1 | GPL-3.0 |
| jean-knapp/image-to-icon-wizard | 1 | CC-BY-4.0 |
| joseph-coston/TLoZ-cursor-pack-linux | 1 | GPL-3.0 |
| koryboc/ImageToIcon | 1 | GPL-3.0 |
| kwfenske/icon-editor-java | 1 | Apache-2.0 |
| lipka/piecon | 1 | NOASSERTION/none-detected |
| luqiming666/AutoUpdater-nsis | 1 | NOASSERTION/none-detected |
| makeable/Notificon | 1 | NOASSERTION |
| matthias-goupil/icon-editor | 1 | NOASSERTION/none-detected |
| matthova/react-icons-converter | 1 | NOASSERTION/none-detected |
| mobzystems/MOBZoom | 1 | NOASSERTION/none-detected |

## Caveats

- Licence strings are those GitHub reports for the repo; `none-detected`/`NOASSERTION` means unknown. Test-suite and icon-pack files may carry other licences. The files are used only for local measurement.
- Many repos are decoder test suites that contain deliberately malformed files; the harness results separate decoder-suite repos from other repos.
- Code search ranks by GitHub relevance, so the sample is biased toward repos that vendor icons and cursor packs; macOS system icons and Windows system icons are not represented.
