# Maintainer automation

## CodeQL migration

The CodeQL Advanced workflow routes from verified Git diffs. Verified prose and funding changes select no languages. Rust, Java/Kotlin and Python changes select their relevant scanner. Workflow, classifier, ambiguous or unknown changes select all four scanners. A weekly run and manual dispatch scan all four languages.

1. With GitHub default setup still enabled, manually dispatch CodeQL Advanced from a reviewed revision with **publish** unchecked. A same-repository PR from a codeql-pilot/ branch also runs in preview mode. The run produces local SARIF and reports counts without uploading results.
2. Compare the four runs, versions, rules and findings with default setup on the same revision. Investigate coverage or result differences and test an editorial PR, a Rust PR, a workflow change and fork behavior before retiring the default setup.
3. Once the comparison is accepted, switch the repository to advanced setup and set the repository Actions variable FOUNDRY_CODEQL_ADVANCED to true. Dispatch a full scan with **publish** checked and verify the uploaded results and alerts before relying on the new configuration.
4. To roll back, remove the activation variable, restore default setup and confirm its full scan reports the four languages.

The advanced workflow never uploads a CodeQL database. The existing required Foundry PR gate is independent from CodeQL.

## Review and contributor commands

- For completed Java proof in a PR, add **Evidence kind: Java**, **Evidence commit: FULL_SHA** and **Negative control: OBSERVED_RESULT** under Verification or Qualification. The advisory checks that the supplied full SHA is the PR head or the tested merge revision. If the control does not apply, state why. Use **Evidence: Pending - REASON** for proof still owed. The maintainer verifies the evidence and applicable gates.
- When a PR against main closes, the cancellation workflow cancels active pull-request runs only when their fork and branch match and the PR link matches, or when the unlinked run matches the current head and no open PR uses that fork and branch. Older runs with ambiguous associations are left alone. The job executes only trusted base code.
- Comment **/claim** on an open unassigned issue with **status:ready-to-claim**. If GitHub accepts the contributor as an assignee, the bot removes the ready label. Maintainers handle Project status, releases of claims and exceptions. A Project status change remains a separate review decision.


## Low-noise maintenance

- Dependabot checks GitHub Actions monthly and groups version updates into at most one open routine PR. Action security updates have their own group and follow the repository's security update policy. Notification delivery follows personal GitHub settings.
- GitHub can generate release notes from the existing PR type labels. Generating notes does not publish a release.
- The PR policy's scope job reports newly broken repository-relative links and Markdown anchors in added or modified Markdown. It does not check external URLs or block the gate.
- The existing gate summary shows the selected CI results and tested revision. On PRs it also shows whether the declared evidence commit matches that PR revision and links to the full checks page.
