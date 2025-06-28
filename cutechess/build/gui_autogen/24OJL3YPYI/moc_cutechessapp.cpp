/****************************************************************************
** Meta object code from reading C++ file 'cutechessapp.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/cutechessapp.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'cutechessapp.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_CuteChessApplication_t {
    QByteArrayData data[15];
    char stringdata0[221];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_CuteChessApplication_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_CuteChessApplication_t qt_meta_stringdata_CuteChessApplication = {
    {
QT_MOC_LITERAL(0, 0, 20), // "CuteChessApplication"
QT_MOC_LITERAL(1, 21, 13), // "newGameWindow"
QT_MOC_LITERAL(2, 35, 11), // "MainWindow*"
QT_MOC_LITERAL(3, 47, 0), // ""
QT_MOC_LITERAL(4, 48, 10), // "ChessGame*"
QT_MOC_LITERAL(5, 59, 4), // "game"
QT_MOC_LITERAL(6, 64, 14), // "newDefaultGame"
QT_MOC_LITERAL(7, 79, 18), // "showSettingsDialog"
QT_MOC_LITERAL(8, 98, 27), // "showTournamentResultsDialog"
QT_MOC_LITERAL(9, 126, 22), // "showGameDatabaseDialog"
QT_MOC_LITERAL(10, 149, 12), // "showGameWall"
QT_MOC_LITERAL(11, 162, 12), // "closeDialogs"
QT_MOC_LITERAL(12, 175, 12), // "onQuitAction"
QT_MOC_LITERAL(13, 188, 18), // "onLastWindowClosed"
QT_MOC_LITERAL(14, 207, 13) // "onAboutToQuit"

    },
    "CuteChessApplication\0newGameWindow\0"
    "MainWindow*\0\0ChessGame*\0game\0"
    "newDefaultGame\0showSettingsDialog\0"
    "showTournamentResultsDialog\0"
    "showGameDatabaseDialog\0showGameWall\0"
    "closeDialogs\0onQuitAction\0onLastWindowClosed\0"
    "onAboutToQuit"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_CuteChessApplication[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
      10,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

 // slots: name, argc, parameters, tag, flags
       1,    1,   64,    3, 0x0a /* Public */,
       6,    0,   67,    3, 0x0a /* Public */,
       7,    0,   68,    3, 0x0a /* Public */,
       8,    0,   69,    3, 0x0a /* Public */,
       9,    0,   70,    3, 0x0a /* Public */,
      10,    0,   71,    3, 0x0a /* Public */,
      11,    0,   72,    3, 0x0a /* Public */,
      12,    0,   73,    3, 0x0a /* Public */,
      13,    0,   74,    3, 0x08 /* Private */,
      14,    0,   75,    3, 0x08 /* Private */,

 // slots: parameters
    0x80000000 | 2, 0x80000000 | 4,    5,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void CuteChessApplication::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<CuteChessApplication *>(_o);
        (void)_t;
        switch (_id) {
        case 0: { MainWindow* _r = _t->newGameWindow((*reinterpret_cast< ChessGame*(*)>(_a[1])));
            if (_a[0]) *reinterpret_cast< MainWindow**>(_a[0]) = std::move(_r); }  break;
        case 1: _t->newDefaultGame(); break;
        case 2: _t->showSettingsDialog(); break;
        case 3: _t->showTournamentResultsDialog(); break;
        case 4: _t->showGameDatabaseDialog(); break;
        case 5: _t->showGameWall(); break;
        case 6: _t->closeDialogs(); break;
        case 7: _t->onQuitAction(); break;
        case 8: _t->onLastWindowClosed(); break;
        case 9: _t->onAboutToQuit(); break;
        default: ;
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject CuteChessApplication::staticMetaObject = { {
    QMetaObject::SuperData::link<QApplication::staticMetaObject>(),
    qt_meta_stringdata_CuteChessApplication.data,
    qt_meta_data_CuteChessApplication,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *CuteChessApplication::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *CuteChessApplication::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_CuteChessApplication.stringdata0))
        return static_cast<void*>(this);
    return QApplication::qt_metacast(_clname);
}

int CuteChessApplication::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QApplication::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 10)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 10;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 10)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 10;
    }
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
