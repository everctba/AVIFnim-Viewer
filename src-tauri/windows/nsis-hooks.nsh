; Hooks e pagina customizada do instalador NSIS do AVIFnim Viewer (Tauri 2).
; - Pagina de associacoes OPT-IN (desmarcadas por padrao) antes da instalacao.
; - Remove as associacoes criadas pelo app ao desinstalar (FR-08).
; - Suporta opt-in silencioso: /ASSOC (todos) ou /ASSOC=avif,webp,gif,pnm

!include nsDialogs.nsh

Var AssocAvif
Var AssocWebp
Var AssocGif
Var AssocPnm
Var AssocCheckAvif
Var AssocCheckWebp
Var AssocCheckGif
Var AssocCheckPnm

LangString ASSOC_TITLE ${LANG_PORTUGUESEBR} "Associacoes de arquivo (opcional)"
LangString ASSOC_TITLE ${LANG_ENGLISH} "File associations (optional)"
LangString ASSOC_SUBTITLE ${LANG_PORTUGUESEBR} "Escolha quais formatos abrir com o AVIFnim Viewer"
LangString ASSOC_SUBTITLE ${LANG_ENGLISH} "Choose which formats open with AVIFnim Viewer"
LangString ASSOC_DESC ${LANG_PORTUGUESEBR} "Tudo desmarcado por padrao. O AVIFnim Viewer nunca altera seus formatos sem permissao. Voce pode mudar isso depois nas configuracoes do app."
LangString ASSOC_DESC ${LANG_ENGLISH} "All unchecked by default. AVIFnim Viewer never changes your formats without permission. You can change this later in the app settings."
LangString ASSOC_AVIF ${LANG_PORTUGUESEBR} "Imagens AVIF (.avif)"
LangString ASSOC_AVIF ${LANG_ENGLISH} "AVIF images (.avif)"
LangString ASSOC_WEBP ${LANG_PORTUGUESEBR} "Imagens WebP (.webp)"
LangString ASSOC_WEBP ${LANG_ENGLISH} "WebP images (.webp)"
LangString ASSOC_GIF ${LANG_PORTUGUESEBR} "Animacoes GIF (.gif)"
LangString ASSOC_GIF ${LANG_ENGLISH} "GIF animations (.gif)"
LangString ASSOC_PNM ${LANG_PORTUGUESEBR} "Imagens PPM/PNM (.ppm .pgm .pbm .pnm)"
LangString ASSOC_PNM ${LANG_ENGLISH} "PPM/PNM images (.ppm .pgm .pbm .pnm)"

Function AssocPage
  ${If} $PassiveMode = 1
    Abort
  ${EndIf}
  !insertmacro MUI_HEADER_TEXT "$(ASSOC_TITLE)" "$(ASSOC_SUBTITLE)"
  nsDialogs::Create 1018
  Pop $0
  ${If} $0 == error
    Abort
  ${EndIf}

  ${NSD_CreateLabel} 0 0 100% 32u "$(ASSOC_DESC)"
  Pop $1

  ${NSD_CreateCheckbox} 0 40u 100% 12u "$(ASSOC_AVIF)"
  Pop $AssocCheckAvif
  ${NSD_SetState} $AssocCheckAvif ${BST_UNCHECKED}

  ${NSD_CreateCheckbox} 0 56u 100% 12u "$(ASSOC_WEBP)"
  Pop $AssocCheckWebp
  ${NSD_SetState} $AssocCheckWebp ${BST_UNCHECKED}

  ${NSD_CreateCheckbox} 0 72u 100% 12u "$(ASSOC_GIF)"
  Pop $AssocCheckGif
  ${NSD_SetState} $AssocCheckGif ${BST_UNCHECKED}

  ${NSD_CreateCheckbox} 0 88u 100% 12u "$(ASSOC_PNM)"
  Pop $AssocCheckPnm
  ${NSD_SetState} $AssocCheckPnm ${BST_UNCHECKED}

  nsDialogs::Show
FunctionEnd

Function AssocPageLeave
  ${NSD_GetState} $AssocCheckAvif $AssocAvif
  ${NSD_GetState} $AssocCheckWebp $AssocWebp
  ${NSD_GetState} $AssocCheckGif $AssocGif
  ${NSD_GetState} $AssocCheckPnm $AssocPnm
FunctionEnd

; Opt-in silencioso (para testes/automacao): /ASSOC ou /ASSOC=avif,webp,gif,pnm
Function ParseAssocArgs
  ${GetOptions} $CMDLINE "/ASSOC=" $R0
  ${If} ${Errors}
    ${GetOptions} $CMDLINE "/ASSOC" $R0
    ${IfNot} ${Errors}
      StrCpy $AssocAvif 1
      StrCpy $AssocWebp 1
      StrCpy $AssocGif 1
      StrCpy $AssocPnm 1
    ${EndIf}
    Return
  ${EndIf}

  ${StrLoc} $R1 $R0 "avif" ">"
  ${If} $R1 != ""
    StrCpy $AssocAvif 1
  ${EndIf}
  ${StrLoc} $R1 $R0 "webp" ">"
  ${If} $R1 != ""
    StrCpy $AssocWebp 1
  ${EndIf}
  ${StrLoc} $R1 $R0 "gif" ">"
  ${If} $R1 != ""
    StrCpy $AssocGif 1
  ${EndIf}
  ${StrLoc} $R1 $R0 "pnm" ">"
  ${If} $R1 != ""
    StrCpy $AssocPnm 1
  ${EndIf}
  ${StrLoc} $R1 $R0 "ppm" ">"
  ${If} $R1 != ""
    StrCpy $AssocPnm 1
  ${EndIf}
FunctionEnd

Function WriteAssocGroup
  ; $R2 = grupo (avif|webp|gif|pnm)
  StrCpy $R3 "AVIFnimViewer.$R2"
  StrCpy $R4 "AVIFnim Viewer - $R2"
  WriteRegStr HKCU "Software\Classes\$R3" "" "$R4"
  WriteRegStr HKCU "Software\Classes\$R3\DefaultIcon" "" "$INSTDIR\${MAINBINARYNAME}.exe,0"
  WriteRegStr HKCU "Software\Classes\$R3\shell\open\command" "" '"$INSTDIR\${MAINBINARYNAME}.exe" "%1"'
FunctionEnd

Function WriteAssociations
  ${If} $AssocAvif == 1
    StrCpy $R2 "avif"
    Call WriteAssocGroup
    WriteRegNone HKCU "Software\Classes\.avif\OpenWithProgids" "AVIFnimViewer.avif"
  ${EndIf}
  ${If} $AssocWebp == 1
    StrCpy $R2 "webp"
    Call WriteAssocGroup
    WriteRegNone HKCU "Software\Classes\.webp\OpenWithProgids" "AVIFnimViewer.webp"
  ${EndIf}
  ${If} $AssocGif == 1
    StrCpy $R2 "gif"
    Call WriteAssocGroup
    WriteRegNone HKCU "Software\Classes\.gif\OpenWithProgids" "AVIFnimViewer.gif"
  ${EndIf}
  ${If} $AssocPnm == 1
    StrCpy $R2 "pnm"
    Call WriteAssocGroup
    WriteRegNone HKCU "Software\Classes\.ppm\OpenWithProgids" "AVIFnimViewer.pnm"
    WriteRegNone HKCU "Software\Classes\.pgm\OpenWithProgids" "AVIFnimViewer.pnm"
    WriteRegNone HKCU "Software\Classes\.pbm\OpenWithProgids" "AVIFnimViewer.pnm"
    WriteRegNone HKCU "Software\Classes\.pnm\OpenWithProgids" "AVIFnimViewer.pnm"
  ${EndIf}
FunctionEnd

; Remove as associacoes criadas pelo app/instalador ao desinstalar (FR-08).
!macro NSIS_HOOK_PREUNINSTALL
  DeleteRegKey HKCU "Software\Classes\AVIFnimViewer.avif"
  DeleteRegKey HKCU "Software\Classes\AVIFnimViewer.webp"
  DeleteRegKey HKCU "Software\Classes\AVIFnimViewer.gif"
  DeleteRegKey HKCU "Software\Classes\AVIFnimViewer.pnm"
  DeleteRegValue HKCU "Software\Classes\.avif\OpenWithProgids" "AVIFnimViewer.avif"
  DeleteRegValue HKCU "Software\Classes\.webp\OpenWithProgids" "AVIFnimViewer.webp"
  DeleteRegValue HKCU "Software\Classes\.gif\OpenWithProgids" "AVIFnimViewer.gif"
  DeleteRegValue HKCU "Software\Classes\.ppm\OpenWithProgids" "AVIFnimViewer.pnm"
  DeleteRegValue HKCU "Software\Classes\.pgm\OpenWithProgids" "AVIFnimViewer.pnm"
  DeleteRegValue HKCU "Software\Classes\.pbm\OpenWithProgids" "AVIFnimViewer.pnm"
  DeleteRegValue HKCU "Software\Classes\.pnm\OpenWithProgids" "AVIFnimViewer.pnm"
!macroend

; Idioma escolhido no instalador -> idioma inicial do app (lido apenas quando ainda
; nao existe settings.json; depois disso a escolha do usuario em Configuracoes manda).
; Aceita /LANG=1033 (English) e /LANG=1046 (PortugueseBR) tambem no modo silencioso,
; onde o MUI ignora /LANG e usa o idioma do sistema.
!macro NSIS_HOOK_POSTINSTALL
  StrCpy $R1 ""
  ${GetOptions} $CMDLINE "/LANG=" $R0
  ${IfNot} ${Errors}
    ${If} "$R0" == "1033"
      StrCpy $R1 "en"
    ${ElseIf} "$R0" == "1046"
      StrCpy $R1 "pt"
    ${ElseIf} "$R0" == "en"
      StrCpy $R1 "en"
    ${ElseIf} "$R0" == "pt"
      StrCpy $R1 "pt"
    ${EndIf}
  ${EndIf}

  ${If} "$R1" == ""
    ${If} $LANGUAGE == ${LANG_ENGLISH}
      StrCpy $R1 "en"
    ${Else}
      StrCpy $R1 "pt"
    ${EndIf}
  ${EndIf}

  WriteRegStr HKCU "Software\AVIFnim" "AppLanguage" "$R1"
!macroend
