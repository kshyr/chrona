package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"sync"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/go-chi/cors"
	"golang.org/x/exp/rand"
)

type JournalEntry struct {
	ID        string    `json:"id"`
	Content   string    `json:"content"`
	CreatedAt time.Time `json:"createdAt"`
	UpdatedAt time.Time `json:"updatedAt"`
}

type InMemoryStore struct {
	sync.RWMutex
	entries map[string]JournalEntry
}

func NewInMemoryStore() *InMemoryStore {
	return &InMemoryStore{
		entries: make(map[string]JournalEntry),
	}
}

func main() {
	store := NewInMemoryStore()
	r := chi.NewRouter()

	r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)

	r.Use(cors.Handler(cors.Options{
		AllowedOrigins:   []string{"http://localhost:3000"},
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowedHeaders:   []string{"Accept", "Authorization", "Content-Type"},
		AllowCredentials: true,
		MaxAge:           300,
	}))

	r.Route("/api/entries", func(r chi.Router) {
		r.Get("/", listEntries(store))
		r.Post("/", createEntry(store))
		r.Put("/{id}", updateEntry(store))
		r.Delete("/{id}", deleteEntry(store))
	})

	log.Println("Starting server on :8080")
	http.ListenAndServe(":8080", r)
}

func listEntries(store *InMemoryStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		fmt.Println(r.Body)
		store.RLock()
		defer store.RUnlock()

		entries := make([]JournalEntry, 0, len(store.entries))
		for _, entry := range store.entries {
			entries = append(entries, entry)
		}

		json.NewEncoder(w).Encode(entries)
	}
}

func createEntry(store *InMemoryStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		var entry JournalEntry
		if err := json.NewDecoder(r.Body).Decode(&entry); err != nil {
			fmt.Errorf("ERRR: %+v\n", err)
			fmt.Errorf("BODY: %+v\n", r.Body)
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}

		entry.ID = generateID()
		entry.CreatedAt = time.Now()
		entry.UpdatedAt = time.Now()

		store.Lock()
		store.entries[entry.ID] = entry
		store.Unlock()

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(entry)
	}
}

func deleteEntry(store *InMemoryStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		id := chi.URLParam(r, "id")

		store.Lock()
		delete(store.entries, id)
		store.Unlock()

		w.WriteHeader(http.StatusNoContent)
	}
}

func updateEntry(store *InMemoryStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		id := chi.URLParam(r, "id")

		var entry JournalEntry
		if err := json.NewDecoder(r.Body).Decode(&entry); err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}

		entry.ID = id
		entry.UpdatedAt = time.Now()

		store.Lock()
		store.entries[id] = entry
		store.Unlock()

		json.NewEncoder(w).Encode(entry)
	}
}

func generateID() string {
	return time.Now().Format("20060102150405") + fmt.Sprintf("%04d", rand.Intn(10000))
}
