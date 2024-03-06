#!/usr/bin/env python3

# Copyright (c) 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import argparse
import glob, os
from gitignore_parser import parse_gitignore

# Anything in ignored paths, including what is inside in case of a directory, will be ignored regardless of gitignore
IGNORED_PATHS = ('pnpm-lock.yaml', 'crates/sui-light-client/example_config/20873329.yaml', 'crates/sui-framework-snapshot/bytecode_snapshot/')
IGNORED_EXTENSIONS = ('svg', 'mv', 'png', 'jpg', 'jpeg', 'gif', 'wasm', 'errmap', 'bcs', 'chk', 'pdf', 'ai', 'mp3', 'wav', 'ico', 'ttf', 'otf', 'woff', 'woff2')

# This mapping allows a simple replace mechanism for certain keywords, it's executed in order
REPLACE_MAP = (
    ('SUI', 'IOTA'),
    ('Sui', 'Iota'),
    ('sui', 'iota'),
    ('MIST', 'MICROS'),
    ('Mist', 'Micros'),
    ('mist', 'micros')
)

# I list of things to not replace if they occur, especially if they are links
CASE_INSENSITIVE_IGNORE_REPLACE = (
    'MystenLabs/sui',
    'suiprivkey',
)

'''
A function that renames all references to SUI and its terms to the corresponding IOTA alternative.
It's highly recommended to run a dry run and manually check the results before actually executing.

The path defaults to the full repo, unless specified otherwise
'''
def rename(path=None, dry_run=True, respect_gitignore=True, skip_filemod=False):
    if not path:
        path = os.path.join(os.path.dirname(__file__), '..')

    path = os.path.abspath(path)
    
    print('\n')
    print(80*'#')

    if dry_run:
        print('# Renaming dry run for for `%s`' % path)
        print('# It is highly recommended to pipe the output of this command to a log file for manual checks')
    else:
        print('# Renaming EXECUTION run for for `%s`' % path)
        print('# It is highly recommended to pipe the output of this command to a log file for manual checks')

    print(80*'#')

    print('# Gathering files to process...\n')

    ignored_paths = [os.path.join(path, f) for f in IGNORED_PATHS]
    gitignore_path = os.path.join(path, '.gitignore')
    
    gitignore = None

    # If there's a gitignore and it's respected, use it, otherwise never ignore
    if respect_gitignore:
        if os.path.exists(gitignore_path):
            gitignore = parse_gitignore(gitignore_path)

    if not gitignore:
        gitignore = lambda x: False
    
    to_process = []
    ignored = []
    renames = []
    suspicious = []

    for filename in glob.iglob(os.path.join(path, '**'), recursive=True):

        for kw, replacement in REPLACE_MAP:
            if kw in os.path.basename(filename):
                renames.append((filename, filename.replace(kw, replacement)))

        if gitignore(filename) or any([filename.startswith(p) for p in ignored_paths]) or any([filename.lower().endswith('.%s' % ext) for ext in IGNORED_EXTENSIONS]):
            ignored.append(filename)
        else:
            to_process.append(filename)

    if not skip_filemod:
        print("# Processing renames for %d files, this might take a while...\n" % len(to_process))

        for fn in to_process:
        
            fh = None
            if os.path.isfile(fn):
                fh = open(fn, 'r+')

            # Skip folders
            if not fh:
                continue

            print(fn)
            
            file_changed = False
            new_content = []
            for line, content in enumerate(fh.readlines()):
                orig_content = content
                content = content
                
                has_changes = False
                for kw, replacement in REPLACE_MAP:
                    if kw not in content:
                        continue
                    
                    ignored_word = False
                    for word in content.split():
                        if '://' in word and kw in word:
                            suspicious.append((fn, line, 'PART_OF_URL', content.strip(), word, '://'))
                            ignored_word = True
                            continue

                        for ignore_part in CASE_INSENSITIVE_IGNORE_REPLACE:
                            if ignore_part.lower() in word.lower():
                                suspicious.append((fn, line, 'IGNORE_LIST', content.strip(), word, ignore_part))
                                ignored_word = True

                    if ignored_word:
                        continue
                    
                    has_changes = True

                    content = content.replace(kw, replacement)

                if has_changes:
                    file_changed = True
                    print('# - File: %s (line: %s): \n#   < %s\n#   > %s\n' % (fn, line, orig_content.strip(), content.strip()))

                new_content.append(content)
            
            if file_changed:
                fcontent = ''.join(new_content)
                #print('\nNew file contents for `%s`: \n```\n%s```\n' % (fn, fcontent))
                if not dry_run:
                    fh.seek(0)
                    fh.write(fcontent)
                    fh.truncate()

            fh.close()
            

        print("# Renaming files and directories as the final step...\n")
        
        # Display ignored non replacements due to ignore list or being suspicious

        if len(suspicious) > 0:
            print('# IGNORED REPLACEMENTS (marked as suspicious), might need manual replacement!\n')

            for sus in suspicious:
                fn, line, reason, content, word, extra = sus
                print('# - %s (line: %s):  %s (%s: %s `%s`)' % (fn, line, content, reason, word, extra))

    # Make sure we sort it by shortest path first so that we don't end up with non-existing not-yet-renamed paths
    '''
    renamed_already = []
    
    for fn, to_fn in sorted(renames, key=lambda x: len(x[0]), reverse=False):
        print('# `%s` -> `%s`' % (fn, to_fn))
        if not dry_run:

            # If we already renamed something before make sure we update the from path as well
            for from_x, to_x in reversed(renamed_already):
                if fn.startswith(from_x + '/'):
                    fn = fn.replace(from_x, to_x, 1)
                if to_fn.startswith(to_x + '/'):
                    to_fn = to_fn.replace(from_x, to_x, 1)
                    break
            
            try:
                os.rename(fn, to_fn)
            except Exception as e:
                print(e, fn, to_fn)
                print("Replaces:")
                print(renamed_already)

            renamed_already.append((fn, to_fn))
    '''
    
    # Dirty loop to try several times in a row instead of implementing logic to deal with partial renames and their exceptions
    
    print('\n\n# Renaming files...\n\n')

    cnt = 0
    while True:

        cnt += 1
        if cnt > 10:
            break

        print('# Round #%d' %cnt) 
    
        renames = []
        errors = []

        for filename in glob.iglob(os.path.join(path, '**'), recursive=True):

            for kw, replacement in REPLACE_MAP:
                if kw in os.path.basename(filename):
                    renames.append((filename, filename.replace(kw, replacement)))

        for fn, to_fn in sorted(renames, key=lambda x: len(x[0]), reverse=False):
            print('# `%s` -> `%s`' % (fn, to_fn))
            if not dry_run:
                try:
                    os.rename(fn, to_fn)
                except Exception as e:
                    errors.append((fn, to_fn))
            
        if len(errors) == 0:
            break

    print("\n# Ignored files: \n# - %s\n" % '\n# - '.join(ignored))

    print("\n\n # DONE!!!\n\n")


def main():
    parser = argparse.ArgumentParser(description='Port a folder recursively to from SUI based naming to IOTA based naming.')

    parser.add_argument('--path', '-p',  help='The path of the folder to process, defaults to the main folder of the project if left out')
    parser.add_argument('--execute', action='store_true', default=False, help='Flag to execute the replacements, if omitted it will just be a dry-run')
    parser.add_argument('--respect-gitignore', action='store_true', default=True, help='Respect what is set in .gitignore to be ignored, defaults to True')
    parser.add_argument('--skip-filemod', action='store_true', default=False, help='Skip file modifcation and only do the rename part')

    args = parser.parse_args()


    rename(path=args.path, dry_run=(not args.execute), respect_gitignore=args.respect_gitignore, skip_filemod=args.skip_filemod)


if __name__ == '__main__':
    main()
