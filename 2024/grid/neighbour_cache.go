package grid

type NeighbourCache struct {
	bounds Bounds
	north  []NeighbourCacheValue
	east   []NeighbourCacheValue
	south  []NeighbourCacheValue
	west   []NeighbourCacheValue
}

type NeighbourCacheValue struct {
	init     bool
	pos      Coord
	inBounds bool
}

func MakeNeighbourCache(bounds Bounds) NeighbourCache {
	north := make([]NeighbourCacheValue, bounds.Size())
	east := make([]NeighbourCacheValue, bounds.Size())
	south := make([]NeighbourCacheValue, bounds.Size())
	west := make([]NeighbourCacheValue, bounds.Size())
	return NeighbourCache{bounds, north, east, south, west}
}

func (cache *NeighbourCache) val(pos Coord, d Direction) *NeighbourCacheValue {
	switch d {
	case North:
		return &cache.north[int(pos)]
	case East:
		return &cache.east[int(pos)]
	case South:
		return &cache.south[int(pos)]
	case West:
		return &cache.west[int(pos)]
	default:
		panic("Unreachable")
	}
}

func (cache *NeighbourCache) Neighbour(pos Coord, d Direction) (Coord, bool) {
	val := cache.val(pos, d)
	if val.init {
		return val.pos, val.inBounds
	} else {
		n, inBounds := cache.bounds.MoveInDirection(pos, d, 1)
		val.init = true
		val.pos = n
		val.inBounds = inBounds
		return n, inBounds
	}
}
