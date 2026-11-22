# Sources

The scanned documents are not kept in the repository. Fetch them from
<https://bitsavers.org/pdf/rand/joss/> into the directories named, if you
want to check a reading. What is kept is what was read out of them: the
transcripts, the notes on how each was measured, and the audit of the
program listings.

The documents this reconstruction was made from, as downloaded on
22 November 2026.

| Directory | Document | File at Bitsavers |
| --- | --- | --- |
| `experience-1965/` | J. C. Shaw, "JOSS: Experience with an Experimental Computing Service for Users at Remote Typewriter Consoles", P-3149, May 1965. Figures 1 and 2, and the one-page summary of 1/12/65. | `P-3149_JOSS_Experience_With_An_Experimental_Computing_Service_For_Users_At_Remote_Typwriter_Consoles_May65.pdf` |
| `examples-1965/` | J. C. Shaw, "JOSS: Examples of the Use of an Experimental On-Line Computing Service", P-3131, April 1965. Eight console pages in two colours. | `P-3131_JOSS_Examples_Of_The_Use_Of_An_Experimental_On-Line_Computing_Service_Apr65.pdf` |
| `designers-view-1964/` | J. C. Shaw, "JOSS: A Designer's View of an Experimental On-Line Computing System", P-2922, August 1964. Figures 3a to 3i. | `P-2922_JOSS_A_Designers_View_Of_An_Experimental_On-Line_Computing_System_Aug64.pdf` |
| `filmed-report-1964/` | C. L. Baker, "JOSS: Scenario of a Filmed Report", RM-4162-PR, June 1964. A session filmed on 21 March 1964, with five facsimiles of typewriter output. | `RM-4162-PR_JOSS_Scenario_Of_A_Filmed_Report_Jun64.pdf` |
| `listing-1965/` | The JOHNNIAC assembly listing of JOSS headed "JOSS 666, JCS 1-9-65" (219 pages). | `JOSS_666_Listing_Jan65.pdf` |
| `listing-undated/` | An earlier assembly of the same program, with an older wording of the messages (55 pages). | `JOSS_Listing.pdf` |
| `program-notes-and-tests/` | Shaw's working papers: a typed summary of the language with handwritten amendments, lists of routines, notes of October and November 1964. No console sessions. | `JOSS_Program_Notes_and_Tests.pdf` |
| `typewriter-system-1962/` | "JOHNNIAC Multiple Typewriter Communication System", D-10507-PR, September 1962. Background. | `D-10507-PR_Johnniac_Multiple_Typewriter_Comm_Sys_Sep62.pdf` |
| `implementation-1966/` | "JOSS: Introduction to the System Implementation", P-3486, November 1966. The later PDP-6 system. Background. | `P-3486_JOSS_Introduction_To_The_System_Implementation_Nov66.pdf` |
| `helpful-assistant-1966/`, `primer-1967/`, `rubrics-1967/` | RM-5058-PR, RM-5220-PR and P-3560: the later JOSS, on the PDP-6. Read only to know what to leave out. | `RM-5058-PR_…`, `RM-5220-PR_…`, `P-3560_…` |

What is here:

- `*/transcripts/*.txt` are the console sessions in each document, read from
  the page images a line at a time, with `U:` in front of what the user typed
  and `J:` in front of what JOSS typed. The columns were measured, not
  judged by eye: the character pitch of each page was fitted and every line
  checked against where the ink is. `NOTES.md` beside them says how, and
  what could not be read with confidence.
- `AUDIT.md` is the reading of the two assembly listings and Shaw's working
  papers: what each is, the forty messages, the tables of words, the limits,
  and what `Go` and `Cancel` do.
- `listing-1965/transcripts/messages.txt` and
  `listing-undated/transcripts/messages.txt` are the message texts, decoded
  from the octal words of the listings.

The sessions the tests replay are in `../transcripts/`, assembled from these.

The scans have no text in them. The `.ocr.txt` files (not kept) were made
with Tesseract a page at a time (`pdftoppm -r 200 -gray`, then
`tesseract --psm 6`). The OCR is good enough for the prose and useless for
the console pages, where the special characters, the underscores of the
forms and the columns are the whole point. Those were read from the images.

All of this reading was done by a coding agent and its sub-agents in one
session. Nobody has checked it against the pages by eye. `../docs/scans/`
has the pages that matter most, for anyone who would like to.
