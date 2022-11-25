package main

import "sync"

type MutexScoreboardManager struct {
	l          sync.RWMutex
	scoreboard map[string]int
}

func NewMutexScoreboardManager() *MutexScoreboardManager {
	return &MutexScoreboardManager{
		scoreboard: map[string]int{},
	}
}
func (msm *MutexScoreboardManager) Update(name string, val int) {
	msm.l.Lock()
	defer msm.l.Unlock()
	msm.scoreboard[name] = val
}
func (msm *MutexScoreboardManager) Read(name string) (int, bool) {
	msm.l.RLock()
	defer msm.l.RUnlock()
	val, ok := msm.scoreboard[name]
	return val, ok
}

func main() {
	msm := NewMutexScoreboardManager()
	msm.Update("foo", 1)
	msm.Update("bar", 2)
	msm.Update("baz", 3)
	msm.Update("qux", 4)
	msm.Update("quux", 5)
	msm.Update("corge", 6)
	msm.Update("grault", 7)
	msm.Update("garply", 8)
	msm.Update("waldo", 9)
	msm.Update("fred", 10)
	msm.Update("plugh", 11)
	msm.Update("xyzzy", 12)
	msm.Update("thud", 13)
}
