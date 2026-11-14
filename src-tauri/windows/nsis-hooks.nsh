; Hooks do instalador NSIS do AVIFnim Viewer (Tauri 2).
; Remove as associacoes opt-in criadas pelo app em HKCU ao desinstalar (FR-08).
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
