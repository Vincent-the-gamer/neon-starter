export default function expect(param: any) {
    const toBe = (result: any) => result === param
    return { toBe }
}