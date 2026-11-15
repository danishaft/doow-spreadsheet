import { LocaleType, mergeLocales, Univer } from '@univerjs/core'
import { FUniver } from '@univerjs/core/facade'
import { UniverFormulaEnginePlugin } from '@univerjs/engine-formula'
import { UniverRenderEnginePlugin } from '@univerjs/engine-render'
import { UniverDocsPlugin } from '@univerjs/docs'
import { UniverSheetsPlugin } from '@univerjs/sheets'
import { UniverSheetsFormulaPlugin } from '@univerjs/sheets-formula'
import { UniverSheetsNumfmtPlugin } from '@univerjs/sheets-numfmt'
import SheetsEnUS from '@univerjs/sheets/locale/en-US'
import '@univerjs/engine-formula/facade'
import '@univerjs/sheets/facade'
import '@univerjs/sheets-formula/facade'
import '@univerjs/sheets-numfmt/facade'

const n = Number(process.argv[2] ?? 100000)
const mode = process.argv[3] ?? 'copied'

function makeUniver(rows) {
  const univer = new Univer({ locale: LocaleType.EN_US, locales: { [LocaleType.EN_US]: mergeLocales(SheetsEnUS) } })
  ;[UniverRenderEnginePlugin, UniverFormulaEnginePlugin, UniverDocsPlugin, UniverSheetsPlugin, UniverSheetsFormulaPlugin, UniverSheetsNumfmtPlugin]
    .forEach((p) => univer.registerPlugin(p))
  const api = FUniver.newAPI(univer)
  const base = api.createWorkbook({})
  const snap = base.save()
  const sid = snap.sheetOrder[0]
  snap.id = `bench-${rows}-${Date.now()}`
  snap.name = 'Bench'
  snap.sheets[sid].rowCount = rows + 10
  snap.sheets[sid].columnCount = 10
  base.dispose()
  const wb = api.createWorkbook(snap)
  return { univer, api, wb, sheet: wb.getActiveSheet() }
}

async function waitFor(sheet, cell, expected, timeoutMs) {
  const t0 = performance.now()
  const deadline = t0 + timeoutMs
  let last = null
  while (performance.now() < deadline) {
    last = sheet.getRange(cell).getValue()
    if (typeof last === 'number' && Math.abs(last - expected) < 1e-6) {
      return { ok: true, ms: performance.now() - t0, value: last }
    }
    await new Promise((r) => setTimeout(r, 25))
  }
  return { ok: false, ms: performance.now() - t0, value: last }
}

const { sheet } = makeUniver(n)
const sum = (n * (n + 1)) / 2

if (mode === 'copied' || mode === 'broad') {
  const values = Array.from({ length: n }, (_, i) => [i + 1])
  const formulas = Array.from({ length: n }, (_, i) =>
    mode === 'copied' ? [`=A${i + 1}*2`] : [`=A${i + 1}*$E$1`]
  )

  const t0 = performance.now()
  if (mode === 'broad') sheet.getRange('E1').setValue(3)
  sheet.getRange(`A1:A${n}`).setValues(values)
  const setValuesMs = performance.now() - t0

  const t1 = performance.now()
  sheet.getRange(`B1:B${n}`).setFormulas(formulas)
  const setFormulasMs = performance.now() - t1

  const expected = (mode === 'copied' ? 2 : 3) * sum
  const t2 = performance.now()
  sheet.getRange('C1').setFormula(`=SUM(B1:B${n})`)
  const calc = await waitFor(sheet, 'C1', expected, n >= 1e6 ? 600000 : 300000)
  const calcMs = calc.ms

  console.log(`case=${mode} n=${n} set_values_ms=${setValuesMs.toFixed(1)} set_formulas_ms=${setFormulasMs.toFixed(1)} calc_ms=${calcMs.toFixed(1)} expected=${expected} got=${calc.value} ok=${calc.ok}`)

  if (mode === 'copied') {
    const t3 = performance.now()
    sheet.getRange('A1').setValue(2)
    const expected2 = expected + 2
    const edit = await waitFor(sheet, 'C1', expected2, n >= 1e6 ? 600000 : 300000)
    console.log(`case=incremental n=${n} edit_plus_recalc_ms=${(performance.now() - t3).toFixed(1)} expected=${expected2} got=${edit.value} ok=${edit.ok}`)
  } else {
    const t3 = performance.now()
    sheet.getRange('E1').setValue(4)
    const expected2 = 4 * sum
    const edit = await waitFor(sheet, 'C1', expected2, n >= 1e6 ? 600000 : 300000)
    console.log(`case=broad_recalc n=${n} edit_plus_recalc_ms=${(performance.now() - t3).toFixed(1)} expected=${expected2} got=${edit.value} ok=${edit.ok}`)
  }
} else {
  console.log('unknown mode')
}
