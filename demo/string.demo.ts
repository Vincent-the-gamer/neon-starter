// @ts-expect-error .node file
import { echo } from "../index.node"
import expect from "./expect"

let echoed = echo("你好呀")  // Echoed text: 你好呀
const result = expect(echoed).toBe("Echoed text: 你好呀") // true

console.log(
    echoed, result
)