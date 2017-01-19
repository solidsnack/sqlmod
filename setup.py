#!/usr/bin/env python
import json
from setuptools import setup
import subprocess

cargo_info = json.loads(subprocess.check_output(['cargo', 'read-manifest']))


conf = dict(name=cargo_info['name'],
            version=cargo_info['version'],
            author='Jason Dusek',
            author_email='jason.dusek@gmail.com',
            url='https://gitlab.com/solidsnack/sqlmod',
            install_requires=['cffi'],
            setup_requires=['pytest-runner', 'setuptools'],
            tests_require=['flake8', 'pytest', 'tox'],
            description='Organize app queries in an annotated SQL file.',
            packages=['sqlmod'],
            zip_safe=False,
            classifiers=['Environment :: Console',
                         'Intended Audience :: Developers',
                         'License :: OSI Approved :: MIT License',
                         'Operating System :: Unix',
                         'Operating System :: POSIX',
                         'Programming Language :: Python',
                         'Programming Language :: Python :: 2.7',
                         'Programming Language :: Python :: 3.5',
                         'Topic :: Software Development',
                         'Development Status :: 4 - Beta'])


if __name__ == '__main__':
    setup(**conf)
