struct Coordinate: Hashable {
    var x: Int
    var y: Int
}

let NEIGHBOURS = [
    Coordinate(x: 1, y: 0),
    Coordinate(x: 0, y: 1),
    Coordinate(x: -1, y: 0),
    Coordinate(x: 0, y: -1)]
