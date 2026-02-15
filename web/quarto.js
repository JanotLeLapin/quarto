import init, { QuartoEngine } from "./pkg/quarto_wasm.js"

const PLAYER_NAMES = ["random", "minimax"]

let quartoPromise
let quarto

export function getPlayerNames() {
  return PLAYER_NAMES
}

/**
 * @returns {Promise<QuartoEngine>}
 */
export async function getQuarto() {
  if (quarto) {
    return quarto
  }

  if (!quartoPromise) {
    quartoPromise = init().then(() => new QuartoEngine())
  }

  quarto = await quartoPromise
  quartoPromise = undefined
  return quarto
}

/**
 * @returns {HTMLElement[]}
 */
export function initBoard() {
  const board = document.getElementById("board")
  const res = []
  for (let i = 0; i < 16; i++) {
    const slot = document.createElement("div")
    slot.classList.add("piece")
    res.push(slot)
    board.appendChild(slot)
  }
  return res
}

/**
 * @returns {HTMLElement}
 */
export function initStaged() {
  const staged = document.getElementById("staged")
  staged.classList.add("piece")
  return staged
}

/**
 * @returns {HTMLElement[]}
 */
export function initStack() {
  const stack = document.getElementById("stack")
  const res = []
  for (let i = 0; i < 16; i++) {
    const slot = document.createElement("div")
    slot.classList.add("piece")
    updateSlot(slot, i)
    res.push(slot)
    stack.appendChild(slot)
  }
  return res
}

export function updateSlot(slot, piece) {
  if (0xFF == piece) {
    slot.classList = "piece"
    return
  }

  if ((piece & 0x01) == 0x01) {
    slot.classList.add("is-light")
  } else {
    slot.classList.add("is-dark")
  }

  if ((piece & 0x02) == 0x02) {
    slot.classList.add("is-square")
  } else {
    slot.classList.add("is-circle")
  }

  if ((piece & 0x04) == 0x04) {
    slot.classList.add("is-tall")
  } else {
    slot.classList.add("is-short")
  }

  if ((piece & 0x08) == 0x08) {
    slot.classList.add("is-hollow")
  }
}
