; The Horse Heroes installer (#192). Compiled by installer/build.ps1, which
; passes the version from Cargo.toml and the folders; do not compile it bare.
; Rules it follows: docs/philosophy/distribution.md, "The installer".

#ifndef AppVersion
  #error AppVersion is not set: run installer/build.ps1
#endif
#ifndef StageDir
  #error StageDir is not set: run installer/build.ps1
#endif
#ifndef OutputDir
  #error OutputDir is not set: run installer/build.ps1
#endif

#define AppName "Horse Heroes"
#define AppExe "horse_heroes.exe"
#define AppUrl "https://github.com/9-7-8/horse-heroes"

[Setup]
; The application ID identifies an existing installation and never changes
; between releases, whatever the visible name becomes (#189).
AppId=9-7-8.HorseHeroes
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher=9-7-8
AppPublisherURL={#AppUrl}
AppSupportURL={#AppUrl}/issues
AppUpdatesURL={#AppUrl}/releases
VersionInfoVersion={#AppVersion}

; For all users, under Program Files, which asks for administrator rights
; (owner, 2026-10-11; it was per user before). {autopf} is Program Files
; when the install is for all users. /CURRENTUSER on the command line
; installs for one user with no rights asked, under their local programs
; folder; the wizard does not offer it.
PrivilegesRequired=admin
PrivilegesRequiredOverridesAllowed=commandline
DefaultDirName={autopf}\{#AppName}
DisableProgramGroupPage=yes
UninstallDisplayIcon={app}\{#AppExe}

; Windows 10 and 11, x64 (#189).
MinVersion=10.0
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

; The installer does not force the game closed.
CloseApplications=no

; The GPL asks nobody to accept it before running the game, so it is shown
; as information, with where the source is, and not as a licence to agree to.
InfoBeforeFile={#StageDir}\SOURCE.txt

OutputDir={#OutputDir}
OutputBaseFilename=HorseHeroes-Setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
; The staged folder is exactly what is installed.
Source: "{#StageDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs

[Icons]
; These start the game until there is a launcher to start instead (#194).
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\{#AppExe}"; WorkingDir: "{app}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExe}"; WorkingDir: "{app}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExe}"; WorkingDir: "{app}"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall skipifsilent
