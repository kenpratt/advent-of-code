package stack

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPush(t *testing.T) {
	st := NewStack[int](10)
	st.Push(3)
	st.Push(5)
	st.Push(7)
	st.Push(9)
	assert.Equal(t, 4, st.Len())
}

func TestPop(t *testing.T) {
	st := NewStack[int](10)
	st.Push(3)
	st.Push(5)
	st.Push(7)
	st.Push(9)
	assert.Equal(t, 9, st.Pop())
	assert.Equal(t, 7, st.Pop())
	assert.Equal(t, 5, st.Pop())
	assert.Equal(t, 3, st.Pop())
	assert.Equal(t, 0, st.Len())
	assert.Panics(t, func() { st.Pop() })
	assert.Equal(t, 0, st.Len())
}

func TestClear(t *testing.T) {
	st := NewStack[int](10)
	st.Push(3)
	st.Push(5)
	st.Push(7)
	st.Push(9)
	st.Clear()
	assert.Equal(t, 0, st.Len())
	assert.Panics(t, func() { st.Pop() })
}

func TestGrow(t *testing.T) {
	st := NewStack[int](2)
	st.Push(3)
	st.Push(5)
	st.Push(7)
	st.Push(9)
	st.Push(11)
	st.Push(13)
	st.Push(15)
	st.Push(17)
	st.Push(19)
	assert.Equal(t, 9, st.Len())
	assert.Equal(t, 19, st.Pop())
	assert.Equal(t, 17, st.Pop())
	assert.Equal(t, 15, st.Pop())
	assert.Equal(t, 13, st.Pop())
	assert.Equal(t, 11, st.Pop())
	assert.Equal(t, 9, st.Pop())
	assert.Equal(t, 7, st.Pop())
	assert.Equal(t, 5, st.Pop())
	assert.Equal(t, 3, st.Pop())
	assert.Equal(t, 0, st.Len())
}
