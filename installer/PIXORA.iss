; Build the application first: cargo build --release -p pixora-photo
; Then run this script with Inno Setup 6 to create PIXORA-Setup.exe.
#define AppName "PIXORA"
#define AppVersion "0.1.0"
#define AppPublisher "JacobProjects"
#define AppExeName "pixora-photo.exe"

[Setup]
AppId={{D9C1A71B-0B6E-4DCE-AF8E-91A0A0000001}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
DefaultDirName={autopf32}\PIXORA
DefaultGroupName=PIXORA
DisableProgramGroupPage=yes
OutputDir=..\dist
OutputBaseFilename=PIXORA-Setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequired=admin
UninstallDisplayIcon={app}\{#AppExeName}
SetupLogging=yes
CloseApplications=yes
RestartApplications=no
Uninstallable=yes
LicenseFile=..\LICENSE

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional shortcuts:"; Flags: unchecked

[Files]
Source: "..\target\release\{#AppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\PIXORA Photo"; Filename: "{app}\{#AppExeName}"
Name: "{autodesktop}\PIXORA Photo"; Filename: "{app}\{#AppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExeName}"; Description: "Launch PIXORA Photo"; Flags: postinstall nowait skipifsilent
