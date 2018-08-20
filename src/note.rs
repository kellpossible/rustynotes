use chrono::{DateTime, Utc};
use std::cell::{RefCell};

/// A note
#[derive(Debug, Clone)]
struct Note {
    /// The title of the note
    title: String,

    /// The time that this note was originally created
    created_time: DateTime<Utc>,

    /// The last time that this note was edited on the server 
    /// (as of the time when the note was last updated from the server).
    /// This is used for identifying conflicts when merging notes
    /// with the server.
    server_edit_time: Option<DateTime<Utc>>,
    
    /// The last time that this note was edited
    edit_time: DateTime<Utc>,

    // The markdown content of the note
    markdown_content: String,
}

impl Note {
    fn new(title: String, markdown_content: String) -> Note {
        Note {
            title: title,
            created_time: Utc::now(),
            server_edit_time: None,
            edit_time: Utc::now(),
            markdown_content: markdown_content
        }
    }

    /// changes to note need to happen immediately after this clone
    fn clone_for_edit(&self) -> Note {
        let mut new_note = self.clone();
        new_note.edit_time = Utc::now();
        return new_note;
    }
}

#[derive(Debug, Clone)]
struct NoteRef {
    /// id of the note which is referenced
    id: u32,

    /// The actual note
    note: Note,
}

impl NoteRef {
    fn new(id: u32, note: Note) -> NoteRef {
        NoteRef {
            id: id,
            note: note
        }
    }
}

trait NoteBackend {
    fn new_note(&self, note: Note) -> NoteRef;
    fn load(&self, id: u32) -> NoteRef;
    fn save(&self, note: NoteRef) -> NoteRef;
}

#[derive(Debug)]
struct DebugNoteBackend {
    notes: RefCell<Vec<NoteRef>>
}

impl DebugNoteBackend {
    fn new() -> DebugNoteBackend {
        DebugNoteBackend {
            notes: RefCell::new(Vec::new())
        }
    }
}

impl NoteBackend for DebugNoteBackend {
    fn new_note(&self, note: Note) -> NoteRef {
        let mut ref_notes = self.notes.borrow_mut();
        let last_id: u32 = match ref_notes.last() {
            Some(last_note) => last_note.id as u32,
            None => 0,
        };

        let new_id = last_id + 1;

        let mut new_note_ref = NoteRef::new(new_id, note);
        new_note_ref.note.server_edit_time = Some(new_note_ref.note.edit_time.clone());
        ref_notes.push(new_note_ref.clone());

        return new_note_ref;
    }

    fn load(&self, id: u32) -> NoteRef {
       let ref_notes = self.notes.borrow(); 
       return ref_notes.iter().find(|&n| n.id == id).unwrap().clone();
    }
    
    fn save(&self, note_ref: NoteRef) -> NoteRef {
        let mut ref_notes = self.notes.borrow_mut();
        let note_pos = ref_notes.iter().position(|n| n.id == note_ref.id).unwrap();
        ref_notes.remove(note_pos);

        let mut new_note_ref = note_ref.clone();
        new_note_ref.note.server_edit_time = Some(new_note_ref.note.edit_time.clone());
        ref_notes.insert(note_pos, new_note_ref.clone());
        return new_note_ref;
    }
}

#[cfg(test)]
mod tests {
    use note::{DebugNoteBackend, NoteBackend, Note};
    use std::{thread, time};

    #[test]
    fn debug_note_backend()
    {
        let backend = DebugNoteBackend::new();

        let new_note = Note::new(String::from("New Note"), String::from("Markdown Content"));
        let new_note_ref = backend.new_note(new_note.clone());

        assert_eq!(String::from("New Note"), new_note_ref.note.title);
        assert_eq!(String::from("Markdown Content"), new_note_ref.note.markdown_content);

        thread::sleep(time::Duration::from_millis(10));

        let mut edited_new_note = new_note.clone_for_edit();
        edited_new_note.markdown_content = String::from("Edited Markdown Content");

        let loaded_note_ref = backend.load(new_note_ref.id);
        let loaded_note = loaded_note_ref.note;
        assert_eq!(new_note_ref.note.title, loaded_note.title);
        assert_eq!(new_note_ref.note.markdown_content, loaded_note.markdown_content);

        assert_eq!(true, loaded_note.server_edit_time.is_some());

        let duration_between_edit_times = edited_new_note.edit_time.signed_duration_since(loaded_note.server_edit_time.unwrap());

        assert_eq!(true, duration_between_edit_times.num_milliseconds() > 0);
    }
}