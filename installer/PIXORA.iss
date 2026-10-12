; Build apps first: cargo build --release -p pixora-photo -p pixora-pdf
#define AppName "PIXORA"
#define AppVersion "0.1.0"
#define AppPublisher "JacobProjects"
#define PhotoExeName "pixora-photo.exe"
#define PdfExeName "pixora-pdf.exe"

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
UninstallDisplayIcon={app}\{#PhotoExeName}
SetupLogging=yes
CloseApplications=yes
RestartApplications=no
Uninstallable=yes
LicenseFile=..\LICENSE

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create desktop shortcuts"; GroupDescription: "Additional shortcuts:"; Flags: unchecked

[Files]
Source: "..\target\release\{#PhotoExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\{#PdfExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\pdfium.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\PIXORA Photo"; Filename: "{app}\{#PhotoExeName}"
Name: "{group}\PIXORA PDF"; Filename: "{app}\{#PdfExeName}"
Name: "{autodesktop}\PIXORA Photo"; Filename: "{app}\{#PhotoExeName}"; Tasks: desktopicon
Name: "{autodesktop}\PIXORA PDF"; Filename: "{app}\{#PdfExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#PhotoExeName}"; Description: "Launch PIXORA Photo"; Flags: postinstall nowait skipifsilent
Filename: "{app}\{#PdfExeName}"; Description: "Launch PIXORA PDF"; Flags: postinstall nowait skipifsilent unchecked
